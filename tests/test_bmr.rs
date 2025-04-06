#[cfg(test)]
mod tests {
    use diet_calc::BmrMethod;
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
        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Male,
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
        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Female,
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
        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Male,
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
        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Female,
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

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Male,
            mass,
            const_height(),
            const_age(),
        );
        assert_eq!(result.unwrap_err(), diet_calc::ERR_MASS);
    }

    #[test]
    fn test_invalid_height() {
        let height = Length::new::<centimeter>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Female,
            const_mass(),
            height,
            const_age(),
        );
        assert_eq!(result.unwrap_err(), diet_calc::ERR_HEIGHT);
    }

    #[test]
    fn test_invalid_age() {
        let age = Time::new::<year>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Female,
            const_mass(),
            const_height(),
            age,
        );
        assert_eq!(result.unwrap_err(), diet_calc::ERR_AGE);
    }

    #[test]
    fn test_invalid_mass_mifflin_st_jeor() {
        let mass = Mass::new::<kilogram>(-10.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Male,
            mass,
            const_height(),
            const_age(),
        );
        assert_eq!(result.unwrap_err(), diet_calc::ERR_MASS);
    }

    #[test]
    fn test_invalid_height_mifflin_st_jeor() {
        let height = Length::new::<centimeter>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Female,
            const_mass(),
            height,
            const_age(),
        );
        assert_eq!(result.unwrap_err(), diet_calc::ERR_HEIGHT);
    }

    #[test]
    fn test_invalid_age_mifflin_st_jeor() {
        let age = Time::new::<year>(-5.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Female,
            const_mass(),
            const_height(),
            age,
        );
        assert_eq!(result.unwrap_err(), diet_calc::ERR_AGE);
    }
}
