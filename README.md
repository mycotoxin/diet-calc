# diet-calc

**diet-calc** is a command-line application written in Rust that allows users to calculate key nutritional and health metrics:

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

BMI is calculated as:

\[ \text{BMI} = \frac{\text{weight (kg)}}{\text{height (m)}^2} \]

BMI categories according to the [World Health Organization (WHO)](https://www.who.int/news-room/fact-sheets/detail/obesity-and-overweight):

- Underweight: <18.5
- Normal weight: 18.5–24.9
- Overweight: 25–29.9
- Obesity: ≥30

### BMR (Basal Metabolic Rate)

BMR calculated using the Mifflin-St Jeor equation ([Mifflin et al., 1990](https://pubmed.ncbi.nlm.nih.gov/2305711/)):

- For men:

\[ \text{BMR} = (10 \times \text{weight in kg}) + (6.25 \times \text{height in cm}) - (5 \times \text{age in years}) + 5 \]

- For women:

\[ \text{BMR} = (10 \times \text{weight in kg}) + (6.25 \times \text{height in cm}) - (5 \times \text{age in years}) - 161 \]

### TDEE (Total Daily Energy Expenditure)

TDEE is BMR multiplied by an activity factor ([source](https://www.ncbi.nlm.nih.gov/books/NBK278961/)):

- Sedentary: BMR × 1.2
- Lightly active: BMR × 1.375
- Moderately active: BMR × 1.55
- Very active: BMR × 1.725
- Super active: BMR × 1.9

### WHtR (Waist-to-Height Ratio)

Calculated as:

\[ \text{WHtR} = \frac{\text{waist circumference (cm)}}{\text{height (cm)}} \]

WHtR values >0.5 indicate increased cardiovascular and metabolic risk ([Browning et al., 2010](https://pubmed.ncbi.nlm.nih.gov/20091484/)).

### BAI (Body Adiposity Index)

BAI estimates body fat percentage:

\[ \text{BAI} = \frac{\text{hip circumference (cm)}}{(\text{height (m)})^{1.5}} - 18 \]

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

## Usage

Follow prompts to input weight, height, age, gender, waist circumference, hip circumference, and activity level. The application outputs BMI, BMR, TDEE, WHtR, and BAI.

### Example

```bash
$ ./diet-calc
Enter your weight in kilograms: 70
Enter your height in centimeters: 175
Enter your age in years: 25
Enter your gender (male/female): male
Enter your waist circumference in centimeters: 80
Enter your hip circumference in centimeters: 95
Enter your activity level (sedentary, lightly active, moderately active, very active, super active): moderately active

Results:
- BMI: 22.9 (Normal weight)
- BMR: 1675 kcal/day
- TDEE: 2596 kcal/day
- WHtR: 0.457 (Healthy)
- BAI: 13.3%
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
