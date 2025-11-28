use core::cell::UnsafeCell;
use core::cmp;
use core::mem::MaybeUninit;
use core::ptr;
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

        let buffer =
            unsafe { MaybeUninit::<[UnsafeCell<MaybeUninit<T>>; N]>::uninit().assume_init() };

        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        head.wrapping_sub(tail) & (N - 1)
    }

    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        let occupied = (head.wrapping_sub(tail)) & (N - 1);
        (N - 1) - occupied
    }

    #[inline]
    pub fn has_room_for(&self, count: usize) -> bool {
        self.remaining_capacity() >= count
    }

    #[inline]
    pub fn push_slice(&self, slice: &[T]) -> Result<(), ()>
    where
        T: Copy,
    {
        let count = slice.len();
        if count == 0 {
            return Ok(());
        }

        if !self.has_room_for(count) {
            return Err(());
        }

        let head = self.head.load(Ordering::Relaxed);

        let to_end = N - head;
        let n1 = cmp::min(count, to_end);
        let n2 = count - n1;

        unsafe {
            let buffer_ptr = self.buffer.as_ptr() as *mut T;

            ptr::copy_nonoverlapping(slice.as_ptr(), buffer_ptr.add(head), n1);

            if n2 > 0 {
                ptr::copy_nonoverlapping(slice.as_ptr().add(n1), buffer_ptr, n2);
            }
        }

        let new_head = (head + count) & (N - 1);
        self.head.store(new_head, Ordering::Release);

        Ok(())
    }

    #[inline]
    pub fn pop_slice(&self, out_slice: &mut [T]) -> usize
    where
        T: Copy,
    {
        let req_count = out_slice.len();
        if req_count == 0 {
            return 0;
        }

        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        let available = (head.wrapping_sub(tail)) & (N - 1);

        let count = cmp::min(req_count, available);

        if count == 0 {
            return 0;
        }

        let to_end = N - tail;
        let n1 = cmp::min(count, to_end);
        let n2 = count - n1;

        unsafe {
            let buffer_ptr = self.buffer.as_ptr() as *const T;

            ptr::copy_nonoverlapping(buffer_ptr.add(tail), out_slice.as_mut_ptr(), n1);

            if n2 > 0 {
                ptr::copy_nonoverlapping(buffer_ptr, out_slice.as_mut_ptr().add(n1), n2);
            }
        }

        let new_tail = (tail + count) & (N - 1);
        self.tail.store(new_tail, Ordering::Release);

        count
    }

    #[inline]
    pub fn push(&self, val: T) -> Result<(), T> {
        if self.remaining_capacity() == 0 {
            return Err(val);
        }
        let head = self.head.load(Ordering::Relaxed);
        unsafe {
            (*self.buffer.get_unchecked(head).get()).write(val);
        }
        self.head.store((head + 1) & (N - 1), Ordering::Release);
        Ok(())
    }

    #[inline]
    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail == head {
            return None;
        }

        let val = unsafe { (*self.buffer.get_unchecked(tail).get()).assume_init_read() };
        self.tail.store((tail + 1) & (N - 1), Ordering::Release);
        Some(val)
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        let tail = self.tail.load(Ordering::Relaxed);

        if tail == self.head.load(Ordering::Acquire) {
            return None;
        }

        unsafe {
            let val_ref = (*self.buffer.get_unchecked(tail).get()).assume_init_ref();
            Some(val_ref)
        }
    }

    #[inline]
    pub fn clear(&self) {
        let head = self.head.load(Ordering::Acquire);
        self.tail.store(head, Ordering::Release);
    }
}
