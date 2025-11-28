use core::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
use core::{
    ops::{Deref, DerefMut},
    slice,
};

#[derive(Clone)]
pub struct VecBlock<T, const N: usize, const S: usize>
where
    T: SimdElement + Default,
    LaneCount<N>: SupportedLaneCount,
{
    buf: [Simd<T, N>; S],
}

impl<T, const N: usize, const S: usize> VecBlock<T, N, S>
where
    T: SimdElement + Default,
    LaneCount<N>: SupportedLaneCount,
{
    pub const LANES: usize = N;

    pub fn splat(value: T) -> Self {
        Self {
            buf: [Simd::splat(value); S],
        }
    }

    pub fn from_array(slice: &[T]) -> Self {
        let (chunks, _) = slice.as_chunks::<N>();

        let buf = core::array::from_fn(|i| Simd::from_array(chunks[i]));
        Self { buf }
    }
    pub fn to_array(&self) -> [T; S * N] {
        let mut out = [T::default(); S * N];
        let (chunks, _) = out.as_chunks_mut::<N>();

        for (i, chunk) in chunks.iter_mut().enumerate() {
            chunk.copy_from_slice(&self.buf[i].to_array());
        }
        out
    }

    pub fn replace(&mut self, other: &Self) {
        self.buf = other.buf;
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        let len = S * N;
        let ptr = self.buf.as_ptr() as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = S * N;
        let ptr = self.buf.as_mut_ptr() as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
}

impl<T, const N: usize, const S: usize> Deref for VecBlock<T, N, S>
where
    T: SimdElement + Default,
    LaneCount<N>: SupportedLaneCount,
{
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize, const S: usize> DerefMut for VecBlock<T, N, S>
where
    T: SimdElement + Default,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<const N: usize, const S: usize> VecBlock<f32, N, S>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    pub fn map_in_place<F>(&mut self, mut op: F)
    where
        F: FnMut(Simd<f32, N>) -> Simd<f32, N>,
    {
        for chunk in self.buf.iter_mut() {
            *chunk = op(*chunk);
        }
    }

    #[inline(always)]
    pub fn map_from<F>(&mut self, source: &Self, mut op: F)
    where
        F: FnMut(Simd<f32, N>) -> Simd<f32, N>,
    {
        for (out_chunk, in_chunk) in self.buf.iter_mut().zip(source.buf.iter()) {
            *out_chunk = op(*in_chunk);
        }
    }

    #[inline(always)]
    pub fn zip_map_in_place<F>(&mut self, other: &Self, mut op: F)
    where
        F: FnMut(Simd<f32, N>, Simd<f32, N>) -> Simd<f32, N>,
    {
        for (a, b) in self.buf.iter_mut().zip(other.buf.iter()) {
            *a = op(*a, *b);
        }
    }

    #[inline(always)]
    pub fn zip_map_from<F>(&mut self, source1: &Self, source2: &Self, mut op: F)
    where
        F: FnMut(Simd<f32, N>, Simd<f32, N>) -> Simd<f32, N>,
    {
        let combined_iter = self
            .buf
            .iter_mut()
            .zip(source1.buf.iter())
            .zip(source2.buf.iter());

        for ((out_chunk, s1_chunk), s2_chunk) in combined_iter {
            *out_chunk = op(*s1_chunk, *s2_chunk);
        }
    }
}
