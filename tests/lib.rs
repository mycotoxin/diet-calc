#[cfg(test)]
mod tests {
    use uom::si::f64::{Length, Mass};
    use uom::si::length::meter;
    use uom::si::mass::kilogram;

    #[test]
    fn shall_calculate_bmi_when_called_bmi() {
        let result = diet_calc::bmi(Mass::new::<kilogram>(60.0), Length::new::<meter>(2.00));
        assert_eq!(result, Ok(15.0));
    }
    #[test]
    fn shall_not_calculate_bmi_when_height_zero() {
        let result = diet_calc::bmi(Mass::new::<kilogram>(60.0), Length::new::<meter>(0.00));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Height must be greater than zero.");
    }
}
