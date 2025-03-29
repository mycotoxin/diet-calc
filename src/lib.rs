use uom::si::f64::{Length, Mass};
use uom::si::length::meter;
use uom::si::mass::kilogram;

pub const ERR_HEIGHT: &str = "Height must be greater than zero.";
pub const ERR_WEIGHT: &str = "Weight must be greater than zero.";

fn validate_positive(value: f64, error_message: &'static str) -> Result<f64, &'static str> {
    if value <= 0.0 {
        return Err(error_message);
    }
    Ok(value)
}

pub fn bmi(weight: Mass, height: Length) -> Result<f64, &'static str> {
    let height_meters = validate_positive(height.get::<meter>(), ERR_HEIGHT)?;
    let weight_kg = validate_positive(weight.get::<kilogram>(), ERR_WEIGHT)?;

    Ok(weight_kg / (height_meters * height_meters))
}
