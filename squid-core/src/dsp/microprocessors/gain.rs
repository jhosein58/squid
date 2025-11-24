use std::simd::{LaneCount, Simd, SupportedLaneCount};

pub struct DbConverter;

impl DbConverter {
    pub fn db_to_amplitude<const N: usize>(db: Simd<f32, N>) -> Simd<f32, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        Simd::splat(10f32) * (db / Simd::splat(20f32))
    }
}

pub struct Gain;

impl Gain {
    pub fn apply<const N: usize>(db: Simd<f32, N>, input: Simd<f32, N>) -> Simd<f32, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        input * DbConverter::db_to_amplitude(db)
    }
}
