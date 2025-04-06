use crate::formulas::constants::{ERR_AGE, ERR_HEIGHT, ERR_MASS, Gender};
use crate::utils::validators::is_positive;
use uom::si::energy::kilocalorie;
use uom::si::f64::{Energy, Length, Mass, Time};
use uom::si::length::centimeter;
use uom::si::mass::kilogram;
use uom::si::time::year;

pub enum BmrMethod {
    HarrisBenedict,
    MifflinStJeor,
}
fn bmr_harris_benedict(
    gender: Gender,
    mass: Mass,
    height: Length,
    age: Time,
) -> Result<Energy, &'static str> {
    let mass_value = mass.get::<kilogram>();
    let height_value = height.get::<centimeter>();
    let age_value = age.get::<year>();

    if !is_positive(mass_value) {
        return Err(ERR_MASS);
    }
    if !is_positive(height_value) {
        return Err(ERR_HEIGHT);
    }
    if !is_positive(age_value) {
        return Err(ERR_AGE);
    }

    let bmr_result = match gender {
        Gender::Male => Energy::new::<kilocalorie>(
            88.362 + (13.397 * mass_value) + (4.799 * height_value) - (5.677 * age_value),
        ),
        Gender::Female => Energy::new::<kilocalorie>(
            447.593 + (9.247 * mass_value) + (3.098 * height_value) - (4.330 * age_value),
        ),
    };

    Ok(bmr_result)
}

fn bmr_mifflin_st_jeor(
    gender: Gender,
    mass: Mass,
    height: Length,
    age: Time,
) -> Result<Energy, &'static str> {
    let mass_value = mass.get::<kilogram>();
    let height_value = height.get::<centimeter>();
    let age_value = age.get::<year>();

    if !is_positive(mass_value) {
        return Err(ERR_MASS);
    }
    if !is_positive(height_value) {
        return Err(ERR_HEIGHT);
    }
    if !is_positive(age_value) {
        return Err(ERR_AGE);
    }

    let bmr_result = match gender {
        Gender::Male => Energy::new::<kilocalorie>(
            (10.0 * mass_value) + (6.25 * height_value) - (5.0 * age_value) + 5.0,
        ),
        Gender::Female => Energy::new::<kilocalorie>(
            (10.0 * mass_value) + (6.25 * height_value) - (5.0 * age_value) - 161.0,
        ),
    };

    Ok(bmr_result)
}

pub fn bmr(
    method: BmrMethod,
    gender: Gender,
    mass: Mass,
    height: Length,
    age: Time,
) -> Result<Energy, &'static str> {
    match method {
        BmrMethod::HarrisBenedict => Ok(bmr_harris_benedict(gender, mass, height, age)?),
        BmrMethod::MifflinStJeor => Ok(bmr_mifflin_st_jeor(gender, mass, height, age)?),
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;

    use float_cmp::assert_approx_eq;
    use uom::si::energy::kilocalorie;
    use uom::si::f64::{Length, Mass, Time};
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
    fn test_bmr_harris_benedict_male() {
        let result = bmr(
            BmrMethod::HarrisBenedict,
            Gender::Male,
            const_mass(),
            const_height(),
            const_age(),
        );

        let expected = 1695.0;
        assert_approx_eq!(
            f64,
            result.unwrap().get::<kilocalorie>(),
            expected,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_bmr_harris_benedict_female() {
        let result = bmr(
            BmrMethod::HarrisBenedict,
            Gender::Female,
            const_mass(),
            const_height(),
            const_age(),
        );

        let expected = 1507.133;
        assert_approx_eq!(
            f64,
            result.unwrap().get::<kilocalorie>(),
            expected,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_bmr_mifflin_st_jeor_male() {
        let result = bmr(
            BmrMethod::MifflinStJeor,
            Gender::Male,
            const_mass(),
            const_height(),
            const_age(),
        );
        let expected = 1648.5;
        assert_approx_eq!(
            f64,
            result.unwrap().get::<kilocalorie>(),
            expected,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_bmr_mifflin_st_jeor_female() {
        let result = bmr(
            BmrMethod::MifflinStJeor,
            Gender::Female,
            const_mass(),
            const_height(),
            const_age(),
        );
        let expected = 1482.0;
        assert_approx_eq!(
            f64,
            result.unwrap().get::<kilocalorie>(),
            expected,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_invalid_mass() {
        let mass = Mass::new::<kilogram>(0.0);

        let result = bmr(
            BmrMethod::HarrisBenedict,
            Gender::Male,
            mass,
            const_height(),
            const_age(),
        );
        assert_eq!(result.unwrap_err(), ERR_MASS);
    }

    #[test]
    fn test_invalid_height() {
        let height = Length::new::<centimeter>(0.0);

        let result = bmr(
            BmrMethod::MifflinStJeor,
            Gender::Female,
            const_mass(),
            height,
            const_age(),
        );
        assert_eq!(result.unwrap_err(), ERR_HEIGHT);
    }

    #[test]
    fn test_invalid_age() {
        let age = Time::new::<year>(0.0);

        let result = bmr(
            BmrMethod::MifflinStJeor,
            Gender::Female,
            const_mass(),
            const_height(),
            age,
        );
        assert_eq!(result.unwrap_err(), ERR_AGE);
    }

    #[test]
    fn test_invalid_mass_mifflin_st_jeor() {
        let mass = Mass::new::<kilogram>(-10.0);

        let result = bmr(
            BmrMethod::HarrisBenedict,
            Gender::Male,
            mass,
            const_height(),
            const_age(),
        );
        assert_eq!(result.unwrap_err(), ERR_MASS);
    }

    #[test]
    fn test_invalid_height_mifflin_st_jeor() {
        let height = Length::new::<centimeter>(0.0);

        let result = bmr(
            BmrMethod::HarrisBenedict,
            Gender::Female,
            const_mass(),
            height,
            const_age(),
        );
        assert_eq!(result.unwrap_err(), ERR_HEIGHT);
    }

    #[test]
    fn test_invalid_age_mifflin_st_jeor() {
        let age = Time::new::<year>(-5.0);

        let result = bmr(
            BmrMethod::HarrisBenedict,
            Gender::Female,
            const_mass(),
            const_height(),
            age,
        );
        assert_eq!(result.unwrap_err(), ERR_AGE);
    }
}
