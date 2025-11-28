use core::simd::{LaneCount, SupportedLaneCount};

pub struct Approx<const N: usize>;

impl<const N: usize> Approx<N> where LaneCount<N>: SupportedLaneCount {}
