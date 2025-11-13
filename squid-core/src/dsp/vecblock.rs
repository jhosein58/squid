use core::{
    ops::{Add, Div, Mul, Sub},
    simd::{LaneCount, Simd, SimdElement, SupportedLaneCount, num::SimdFloat},
};
use sleef::f32x::sin_u10_deterministic;

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
    pub fn to_array(&self) -> [T; S] {
        let mut out = [T::default(); S];
        let (chunks, _) = out.as_chunks_mut::<N>();

        for (i, chunk) in chunks.iter_mut().enumerate() {
            chunk.copy_from_slice(&self.buf[i].to_array());
        }
        out
    }
}

impl<const N: usize, const S: usize> VecBlock<f32, N, S>
where
    LaneCount<N>: SupportedLaneCount,
{
    pub fn in_place<O: VecOp<Simd<f32, N>, N>>(mut self, other: &Self) -> Self {
        for (a, b) in self.buf.iter_mut().zip(other.buf.iter()) {
            *a = O::apply(*a, *b);
        }
        self
    }

    pub fn sin(mut self) -> Self {
        for d in self.buf.iter_mut() {
            *d = sin_u10_deterministic(*d);
        }
        self
    }
}

pub trait VecOp<T, const N: usize>
where
    T: SimdFloat,
{
    fn apply(a: T, b: T) -> T;
}

macro_rules! impl_vecop {
    ($name:ident, $bound:ident, $closure:expr) => {
        pub struct $name;

        impl<T, const N: usize> VecOp<T, N> for $name
        where
            T: SimdFloat + core::ops::$bound<T, Output = T>,
        {
            fn apply(a: T, b: T) -> T {
                ($closure)(a, b)
            }
        }
    };
}

impl_vecop!(VecAdd, Add, |a, b| a + b);
impl_vecop!(VecSub, Sub, |a, b| a - b);
impl_vecop!(VecMul, Mul, |a, b| a * b);
impl_vecop!(VecDiv, Div, |a, b| a / b);

impl<const N: usize, const S: usize> Add for VecBlock<f32, N, S>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.in_place::<VecAdd>(&other)
    }
}

impl<const N: usize, const S: usize> Sub for VecBlock<f32, N, S>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.in_place::<VecSub>(&other)
    }
}

impl<const N: usize, const S: usize> Mul for VecBlock<f32, N, S>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.in_place::<VecMul>(&other)
    }
}

impl<const N: usize, const S: usize> Div for VecBlock<f32, N, S>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.in_place::<VecDiv>(&other)
    }
}
