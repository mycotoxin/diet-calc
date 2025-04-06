use crate::formulas::bmr::{BmrMethod, bmr};
use crate::formulas::constants::Gender;
use uom::si::f32::{Energy, Length, Mass, Time};

const ERR_ACTIVITY: &'static str = "Activity level must be positive.";

pub fn tdee(
    bmr_method: BmrMethod,
    gender: Gender,
    mass: Mass,
    height: Length,
    age: Time,
    activity_level: f32,
) -> Result<Energy, &'static str> {
    if activity_level <= 0.0 {
        return Err(ERR_ACTIVITY);
    }

    let basal_metabolic_rate = bmr(bmr_method, gender, mass, height, age)?;
    Ok(basal_metabolic_rate * activity_level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formulas::constants::ERR_MASS;
    use float_cmp::assert_approx_eq;
    use uom::si::energy::kilocalorie;
    use uom::si::length::centimeter;
    use uom::si::mass::kilogram;
    use uom::si::time::year;

    fn const_mass() -> Mass {
        Mass::new::<kilogram>(70.0)
    }
    fn const_height() -> Length {
        Length::new::<centimeter>(175.0)
    }
    fn const_age() -> Time {
        Time::new::<year>(30.0)
    }

    #[test]
    fn test_tdee() {
        let result = tdee(
            BmrMethod::HarrisBenedict,
            Gender::Male,
            const_mass(),
            const_height(),
            const_age(),
            1.55,
        );
        assert!(result.is_ok());
        assert_approx_eq!(
            f32,
            result.unwrap().get::<kilocalorie>(),
            2625.0,
            epsilon = 10.0
        );
    }

    #[test]
    fn test_invalid_activity_level() {
        let result = tdee(
            BmrMethod::HarrisBenedict,
            Gender::Male,
            const_mass(),
            const_height(),
            const_age(),
            -1.0,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ERR_ACTIVITY);
    }

    #[test]
    fn test_invalid_mass_tdee() {
        let result = tdee(
            BmrMethod::MifflinStJeor,
            Gender::Female,
            Mass::new::<kilogram>(0.0),
            const_height(),
            const_age(),
            1.2,
        );
        assert_eq!(result, Err(ERR_MASS));
    }
}
