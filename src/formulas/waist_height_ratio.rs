use crate::formulas::constants::{ERR_HEIGHT, ERR_WAIST};
use crate::utils::validators::is_positive;
use uom::si::f32::Length;
use uom::si::length::centimeter;

pub fn whtr(waist: Length, height: Length) -> Result<f32, &'static str> {
    let waist_cm = waist.get::<centimeter>();
    let height_cm = height.get::<centimeter>();

    if !is_positive(waist_cm) {
        return Err(ERR_WAIST);
    }
    if !is_positive(height_cm) {
        return Err(ERR_HEIGHT);
    }

    Ok(waist_cm / height_cm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::length::centimeter;

    #[test]
    fn calculates_correct_whtr() {
        let waist = Length::new::<centimeter>(80.0);
        let height = Length::new::<centimeter>(160.0);

        let result = whtr(waist, height);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.5);
    }

    #[test]
    fn returns_err_if_waist_is_invalid() {
        let waist = Length::new::<centimeter>(0.0);
        let height = Length::new::<centimeter>(160.0);

        let result = whtr(waist, height);
        assert_eq!(result.unwrap_err(), ERR_WAIST);
    }

    #[test]
    fn returns_err_if_height_is_invalid() {
        let waist = Length::new::<centimeter>(80.0);
        let height = Length::new::<centimeter>(-5.0);

        let result = whtr(waist, height);
        assert_eq!(result.unwrap_err(), ERR_HEIGHT);
    }
}
