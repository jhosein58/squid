use core::simd::{LaneCount, Simd, SupportedLaneCount, cmp::SimdPartialOrd};

pub struct PolyBlep;

impl PolyBlep {
    #[inline(always)]
    pub fn calc_blep_residual<const N: usize>(phase: Simd<f32, N>, dt: Simd<f32, N>) -> Simd<f32, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let one = Simd::splat(1.0);
        let two = Simd::splat(2.0);
        let zero = Simd::splat(0.0);

        let inv_dt = one / dt;
        let mask_start = phase.simd_lt(dt);
        let t_start = phase * inv_dt;
        let blep_start = (two * t_start) - (t_start * t_start) - one;
        let mask_end = phase.simd_gt(one - dt);
        let t_end = (phase - one) * inv_dt;
        let blep_end = (t_end * t_end) + (two * t_end) + one;
        let result = mask_start.select(blep_start, zero);

        mask_end.select(blep_end, result)
    }
}
