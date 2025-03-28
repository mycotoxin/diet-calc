use uom::si::f64::{Length, Mass};
use uom::si::length::meter;
use uom::si::mass::kilogram;

pub fn bmi(weight: Mass, height: Length) -> Result<f64, &'static str> {
    let height = height.get::<meter>();
    if height <= 0.0 {
        return Err("Height must be greater than zero.");
    }
    Ok(weight.get::<kilogram>() / (height * height))
}
