use crate::formulas::constants::{ERR_HEIGHT, ERR_HIP};
use crate::utils::validators::is_positive;
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter};

/// Body Adiposity Index (BAI) — Oblicza przybliżony procent tkanki tłuszczowej.
///
/// # Wzór
/// BAI = (Hip circumference [m] / Height [m]^1.5) - 18
pub fn bai(hip: Length, height: Length) -> Result<f32, &'static str> {
    let hip_cm = hip.get::<centimeter>();
    let height_m = height.get::<meter>();

    if !is_positive(hip_cm) {
        return Err(ERR_HIP);
    }
    if !is_positive(height_m) {
        return Err(ERR_HEIGHT);
    }

    Ok((hip_cm / height_m.powf(1.5)) - 18.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use uom::si::length::centimeter;

    #[test]
    fn calculates_bai_correctly() {
        let hip = Length::new::<centimeter>(90.0);
        let height = Length::new::<centimeter>(170.0);

        let result = bai(hip, height);
        assert!(result.is_ok());
        
        // wynik przybliżony około 21.1
        assert_approx_eq!(f32, result.unwrap(), 22.6, epsilon = 0.1);
    }

    #[test]
    fn returns_err_if_hip_invalid() {
        let hip = Length::new::<centimeter>(0.0);
        let height = Length::new::<centimeter>(170.0);

        let result = bai(hip, height);
        assert_eq!(result.unwrap_err(), ERR_HIP);
    }

    #[test]
    fn returns_err_if_height_invalid() {
        let hip = Length::new::<centimeter>(90.0);
        let height = Length::new::<centimeter>(-1.0);

        let result = bai(hip, height);
        assert_eq!(result.unwrap_err(), ERR_HEIGHT);
    }
}