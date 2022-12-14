/// Simple Lineal Interpolation
///
/// a and b = values to interpolate
/// t = value between 0 and 1
/// returns an interpolated value proportional to t between a and b
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    (1_f32 - t) * a + b * t
}

/// Inverse Lineal Interpolation
///
/// Returns the t (value between 0 and 1) that would result from interpolation of a to b to return v
pub fn ilerp_f32(a: f32, b: f32, v: f32) -> f32 {
    (v - a) / (b - a)
}

/// Remapping
///
/// i_min to i_max = the input range
/// o_min to o_max = the output range
/// v = value between iMin and iMax
/// returns the equivalent value between o_min and o_max
pub fn remap_f32(i_min: f32, i_max: f32, o_min: f32, o_max: f32, v: f32) -> f32 {
    let t = ilerp_f32(i_min, i_max, v);
    lerp_f32(o_min, o_max, t)
}
