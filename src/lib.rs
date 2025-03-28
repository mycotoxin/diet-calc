use uom::si::f64::{Length, Mass};
use uom::si::length::meter;
use uom::si::mass::kilogram;

pub fn bmi(weight: Mass, height: Length) -> f64 {
    let height = height.get::<meter>();
    weight.get::<kilogram>() / (height * height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shall_calculate_bmi_when_called_bmi() {
        let result = bmi(Mass::new::<kilogram>(60.0), Length::new::<meter>(2.00));
        assert_eq!(result, 15.0);
    }
}
