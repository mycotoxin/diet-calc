pub mod utils;

use uom::si::energy::kilocalorie;
use uom::si::f64::{Energy, Length, Mass, Time};
use uom::si::length::centimeter;
use uom::si::length::meter;
use uom::si::mass::kilogram;
use uom::si::time::year;
use utils::validators::validate_positive;

pub const ERR_HEIGHT: &str = "Height must be greater than zero.";
pub const ERR_MASS: &str = "Weight must be greater than zero.";
pub const ERR_AGE: &str = "Age must be greater than zero.";
pub enum Gender {
    Male,
    Female,
}

pub fn bmi(weight: Mass, height: Length) -> Result<f64, &'static str> {
    let height_meters = validate_positive(height.get::<meter>(), ERR_HEIGHT)?;
    let weight_kg = validate_positive(weight.get::<kilogram>(), ERR_MASS)?;

    Ok(weight_kg / (height_meters * height_meters))
}

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
    let mass_kg = validate_positive(mass.get::<kilogram>(), ERR_MASS)?;
    let height_cm = validate_positive(height.get::<centimeter>(), ERR_HEIGHT)?;
    let age_years = validate_positive(age.get::<year>(), ERR_AGE)?;

    let bmr_result = match gender {
        Gender::Male => Energy::new::<kilocalorie>(
            88.362 + (13.397 * mass_kg) + (4.799 * height_cm) - (5.677 * age_years),
        ),
        Gender::Female => Energy::new::<kilocalorie>(
            447.593 + (9.247 * mass_kg) + (3.098 * height_cm) - (4.330 * age_years),
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
    let mass_kg = validate_positive(mass.get::<kilogram>(), ERR_MASS)?;
    let height_cm = validate_positive(height.get::<centimeter>(), ERR_HEIGHT)?;
    let age_years = validate_positive(age.get::<year>(), ERR_AGE)?;

    let bmr_result = match gender {
        Gender::Male => Energy::new::<kilocalorie>(
            (10.0 * mass_kg) + (6.25 * height_cm) - (5.0 * age_years) + 5.0,
        ),
        Gender::Female => Energy::new::<kilocalorie>(
            (10.0 * mass_kg) + (6.25 * height_cm) - (5.0 * age_years) - 161.0,
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
