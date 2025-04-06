#[cfg(test)]
mod tests {
    use diet_calc::BmrMethod;
    use float_cmp::assert_approx_eq;
    use uom::si::energy::kilocalorie;
    use uom::si::f64::{Length, Mass, Time};
    use uom::si::length::centimeter;
    use uom::si::mass::kilogram;
    use uom::si::time::year;

    fn default_test_input() -> (Mass, Length, Time) {
        (
            Mass::new::<kilogram>(70.0),
            Length::new::<centimeter>(175.0),
            Time::new::<year>(30.0),
        )
    }

    #[test]
    fn test_bmr_harris_benedict_male() {
        let (mass, height, age) = default_test_input();
        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Male,
            mass,
            height,
            age,
        );

        assert!(!result.is_err());
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
        let (mass, height, age) = default_test_input();
        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Female,
            mass,
            height,
            age,
        );

        assert!(result.is_ok());
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
        let (mass, height, age) = default_test_input();
        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Male,
            mass,
            height,
            age,
        );
        assert!(result.is_ok());
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
        let (mass, height, age) = default_test_input();
        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Female,
            mass,
            height,
            age,
        );
        assert!(result.is_ok());
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
        let (_, height, age) = default_test_input();
        let mass = Mass::new::<kilogram>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Male,
            mass,
            height,
            age,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), diet_calc::ERR_MASS);
    }

    #[test]
    fn test_invalid_height() {
        let (mass, _, age) = default_test_input();
        let height = Length::new::<centimeter>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Female,
            mass,
            height,
            age,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), diet_calc::ERR_HEIGHT);
    }

    #[test]
    fn test_invalid_age() {
        let (mass, height, _) = default_test_input();
        let age = Time::new::<year>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::MifflinStJeor,
            diet_calc::Gender::Female,
            mass,
            height,
            age,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), diet_calc::ERR_AGE);
    }

    #[test]
    fn test_invalid_mass_mifflin_st_jeor() {
        let (_, height, age) = default_test_input();
        let mass = Mass::new::<kilogram>(-10.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Male,
            mass,
            height,
            age,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), diet_calc::ERR_MASS);
    }

    #[test]
    fn test_invalid_height_mifflin_st_jeor() {
        let (mass, _, age) = default_test_input();
        let height = Length::new::<centimeter>(0.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Female,
            mass,
            height,
            age,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), diet_calc::ERR_HEIGHT);
    }

    #[test]
    fn test_invalid_age_mifflin_st_jeor() {
        let (mass, height, _) = default_test_input();
        let age = Time::new::<year>(-5.0);

        let result = diet_calc::bmr(
            BmrMethod::HarrisBenedict,
            diet_calc::Gender::Female,
            mass,
            height,
            age,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), diet_calc::ERR_AGE);
    }
}
