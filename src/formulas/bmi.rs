use crate::formulas::constants::{ERR_HEIGHT, ERR_MASS};
use crate::utils::validators::is_positive;
use uom::si::f32::{Length, Mass};
use uom::si::length::meter;
use uom::si::mass::kilogram;

pub fn bmi(mass: Mass, height: Length) -> Result<f32, &'static str> {
    let mass = mass.get::<kilogram>();
    let height = height.get::<meter>();
    if !is_positive(mass) {
        return Err(ERR_MASS);
    }
    if !is_positive(height) {
        return Err(ERR_HEIGHT);
    }

    Ok(mass / (height * height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::f32::{Length, Mass};
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
