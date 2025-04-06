# diet-calc
[](https://crates.io/crates/diet-calc)[](https://github.com/<your-username>/diet-calc/blob/main/LICENSE)
## Description
**diet-calc** is a Rust library providing easy-to-use functions designed for calculating basic nutritional indicators such as Body Mass Index (BMI) and Basal Metabolic Rate (BMR). The library uses uom (Units of Measurement) to handle type-safe physical quantities like mass, length, and energy, preventing unit-related calculation mistakes.
## Provided functionality
- Body Mass Index (BMI) calculation
- Basal Metabolic Rate (BMR) calculation, using:
    - Harris-Benedict formula
    - Mifflin-St Jeor formula

## Formulas used
### BMI (Body Mass Index)
BMI is calculated using weight (mass) and height:
BMI[ = \frac{\text{mass (kg)}}{\text{height (m)}^2} ]
Reference: [WHO BMI classification](https://www.who.int/data/gho/data/themes/topics/topic-details/GHO/body-mass-index)
### BMR (Basal Metabolic Rate)
#### Harris-Benedict formula (1919)
This classical formula approximates daily caloric needs:
latex_unknown_tag
``` 
Men:
BMR = 88.362 + (13.397 × weight[kg]) + (4.799 × height[cm]) − (5.677 × age[years])

Women:
BMR = 447.593 + (9.247 × weight[kg]) + (3.098 × height[cm]) − (4.330 × age[years])
```
Reference: [Original Harris-Benedict Publication](https://pubmed.ncbi.nlm.nih.gov/16576330/)
#### Mifflin-St Jeor formula (1990)
Regarded as more accurate for modern dietary requirements estimation:
``` 
Men:
BMR = (10 × weight[kg]) + (6.25 × height[cm]) − (5 × age[years]) + 5

Women:
BMR = (10 × weight[kg]) + (6.25 × height[cm]) − (5 × age[years]) − 161
```
Reference: [Mifflin MD, St Jeor ST, et al. (1990)](https://pubmed.ncbi.nlm.nih.gov/2305711/)
## Safety checks
All calculation functions verify input data for correctness. Input values such as weight, height, and age must always be greater than zero.
## Usage example
``` rust
use diet_calc::{bmi, bmr, BmrMethod, Gender};
use uom::si::f64::{Mass, Length, Time};
use uom::si::mass::kilogram;
use uom::si::length::meter;
use uom::si::time::year;
use uom::si::energy::kilocalorie;

fn main() {
    let mass = Mass::new::<kilogram>(70.0);
    let height = Length::new::<meter>(1.75);
    let age = Time::new::<year>(30.0);

    match bmi(mass, height) {
        Ok(result) => println!("BMI: {:.2}", result),
        Err(e) => println!("Error calculating BMI: {}", e),
    }

    match bmr(BmrMethod::MifflinStJeor, Gender::Male, mass, height, age) {
        Ok(result) => println!("BMR (Mifflin-St Jeor): {:.2} kcal", result.get::<kilocalorie>()),
        Err(e) => println!("Error calculating BMR: {}", e),
    }
}
```
## Compatibility
The library is compatible with Rust 1.85.1 or higher.
## License
Licensed under either of these licenses at your option:
- MIT License
- Apache License 2.0

## Installation
Include the following in your `Cargo.toml` file:
``` toml
[dependencies]
diet-calc = "0.1"
```
For any questions, feature requests, or bug reports, feel free to visit the project repository!
[GitHub Project: diet-calc](https://github.com/<your-username>/diet-calc)
