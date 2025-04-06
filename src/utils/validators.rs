pub fn is_positive<T: PartialOrd<f32>>(value: T) -> bool {
    if value > 0.0 {
        return true;
    }
    false
}
