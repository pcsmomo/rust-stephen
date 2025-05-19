/// Linear interpolation between two values.
///
/// `lerp` calculates a value between `start` and `end` based on parameter `t`:
/// - When t = 0.0, returns `start`
/// - When t = 1.0, returns `end`
/// - When 0.0 < t < 1.0, returns a proportional value between `start` and `end`
///
/// This is commonly used in graphics to blend between two values (colors, positions, etc.)
/// based on a normalized parameter.
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + t * end
}
