pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}
