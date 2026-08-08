/// Clamps a value between min and max.
///
/// @brief Clamp value to range
/// @param value Input value
/// @param min Minimum bound
/// @param max Maximum bound
/// @return Clamped value
/// @since 0.1.0
/// @author Miruamel
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linearly interpolates between two values.
///
/// @brief Linear interpolation
/// @param start Start value
/// @param end End value
/// @param t Interpolation factor (0.0 - 1.0)
/// @return Interpolated value
/// @since 0.1.0
/// @author Miruamel
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}
