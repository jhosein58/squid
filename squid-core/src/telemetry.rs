use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct FixedSpscQueue<T, const N: usize> {
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T: Send, const N: usize> Send for FixedSpscQueue<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for FixedSpscQueue<T, N> {}

impl<T, const N: usize> FixedSpscQueue<T, N> {
    pub fn new() -> Self {
        assert!(N.is_power_of_two(), "N must be a power of two");
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    #[inline]
    fn next_index(&self, index: usize) -> usize {
        (index + 1) & (N - 1)
    }

    #[inline]
    pub fn push(&self, val: T) -> Result<(), T> {
        let head = self.head.load(Ordering::Relaxed);
        let next_head = self.next_index(head);

        if next_head == self.tail.load(Ordering::Acquire) {
            return Err(val);
        }

        unsafe {
            (*self.buffer.get_unchecked(head).get()).write(val);
        }

        self.head.store(next_head, Ordering::Release);
        Ok(())
    }

    #[inline]
    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);

        if tail == self.head.load(Ordering::Acquire) {
            return None;
        }

        let val = unsafe { (*self.buffer.get_unchecked(tail).get()).assume_init_read() };
        self.tail.store(self.next_index(tail), Ordering::Release);

        Some(val)
    }

    pub fn capacity(&self) -> usize {
        N - 1
    }

    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        (head.wrapping_sub(tail)) & (N - 1)
    }

    #[inline]
    pub fn clear(&self) {
        self.head.store(0, Ordering::Release);
        self.tail.store(0, Ordering::Release);
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        let tail = self.tail.load(Ordering::Relaxed);
        if tail == self.head.load(Ordering::Acquire) {
            return None;
        }
        let val_ref = unsafe { (*self.buffer.get_unchecked(tail).get()).assume_init_ref() };
        Some(val_ref)
    }
}

impl<T, const N: usize> Drop for FixedSpscQueue<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
