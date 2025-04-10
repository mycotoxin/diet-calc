# diet-calc

**diet-calc** is a Rust library that allows users to calculate key nutritional and health metrics:

- Body Mass Index (**BMI**)
- Basal Metabolic Rate (**BMR**)
- Total Daily Energy Expenditure (**TDEE**)
- Waist-to-Height Ratio (**WHtR**)
- Body Adiposity Index (**BAI**)

These metrics assist dietitians and health-conscious individuals in assessing body composition and daily caloric requirements.

## Features

- **BMI Calculation**: Evaluates body weight status.
- **BMR Calculation**: Calculates resting caloric needs.
- **TDEE Calculation**: Estimates daily caloric expenditure based on activity level.
- **WHtR Calculation**: Assesses abdominal obesity risk.
- **BAI Calculation**: Estimates body fat percentage without direct body composition measurement.

## Formulas Used

### BMI (Body Mass Index)

BMI categories according to the [World Health Organization (WHO)](https://www.who.int/news-room/fact-sheets/detail/obesity-and-overweight):

- Underweight: <18.5
- Normal weight: 18.5–24.9
- Overweight: 25–29.9
- Obesity: ≥30

### BMR (Basal Metabolic Rate)

BMR calculated using the Mifflin-St Jeor equation ([Mifflin et al., 1990](https://pubmed.ncbi.nlm.nih.gov/2305711/)):

### TDEE (Total Daily Energy Expenditure)

TDEE is BMR multiplied by an activity factor ([source](https://www.ncbi.nlm.nih.gov/books/NBK278961/))

- Sedentary: BMR × 1.2
- Lightly active: BMR × 1.375
- Moderately active: BMR × 1.55
- Very active: BMR × 1.725
- Super active: BMR × 1.9

### WHtR (Waist-to-Height Ratio)

WHtR values >0.5 indicate increased cardiovascular and metabolic risk ([Browning et al., 2010](https://pubmed.ncbi.nlm.nih.gov/20091484/)).

### BAI (Body Adiposity Index)

BAI estimates body fat percentage:


Formula from [Bergman et al., 2011](https://pubmed.ncbi.nlm.nih.gov/21372804/).

## System Requirements

- Rust programming language (1.65+)

## Installation

Clone, build, and run the project:

```bash
git clone https://github.com/mycotoxin/diet-calc.git
cd diet-calc
cargo build --release
./target/release/diet-calc
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
