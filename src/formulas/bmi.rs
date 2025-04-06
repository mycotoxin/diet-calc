use crate::formulas::constants::{ERR_HEIGHT, ERR_MASS};
use crate::utils::validators::validate_positive;
use uom::si::f64::{Length, Mass};
use uom::si::length::meter;
use uom::si::mass::kilogram;

pub fn bmi(weight: Mass, height: Length) -> Result<f64, &'static str> {
    let height_meters = validate_positive(height.get::<meter>(), ERR_HEIGHT)?;
    let weight_kg = validate_positive(weight.get::<kilogram>(), ERR_MASS)?;

    Ok(weight_kg / (height_meters * height_meters))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::f64::{Length, Mass};
    use uom::si::length::meter;
    use uom::si::mass::kilogram;

    #[test]
    fn shall_calculate_bmi_when_called_bmi() {
        let result = bmi(Mass::new::<kilogram>(60.0), Length::new::<meter>(2.00));
        assert_eq!(result, Ok(15.0));
    }
    #[test]
    fn shall_not_calculate_bmi_when_height_zero() {
        let result = bmi(Mass::new::<kilogram>(60.0), Length::new::<meter>(0.00));
        assert_eq!(result.unwrap_err(), ERR_HEIGHT);
    }
    #[test]
    fn shall_not_calculate_bmi_when_mass_zero() {
        let result = bmi(Mass::new::<kilogram>(0.0), Length::new::<meter>(2.00));
        assert_eq!(result.unwrap_err(), ERR_MASS);
    }
}
