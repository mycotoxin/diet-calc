pub fn validate_positive(value: f64, error_message: &'static str) -> Result<f64, &'static str> {
    if value <= 0.0 {
        return Err(error_message);
    }
    Ok(value)
}
