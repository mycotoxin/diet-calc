## Provided functionality

- Body Mass Index (BMI) calculation
- Basal Metabolic Rate (BMR) calculation, using:
  - Harris-Benedict formula
  - Mifflin-St Jeor formula
- Total Daily Energy Expenditure (TDEE) estimation, which factors in activity levels to determine daily caloric needs.

## Formulas used

### BMI (Body Mass Index)

BMI is calculated using weight (mass) and height:
BMI[ = \frac{\text{mass (kg)}}{\text{height (m)}^2} ]
Reference: [WHO BMI classification](https://www.who.int/data/gho/data/themes/topics/topic-details/GHO/body-mass-index)

### BMR (Basal Metabolic Rate)

#### Harris-Benedict formula (1919)

This classical formula approximates daily caloric needs:

Reference: [Original Harris-Benedict Publication](https://pubmed.ncbi.nlm.nih.gov/16576330/)

#### Mifflin-St Jeor formula (1990)

Reference: [Mifflin MD, St Jeor ST, et al. (1990)](https://pubmed.ncbi.nlm.nih.gov/2305711/)

### TDEE (Total Daily Energy Expenditure)

TDEE is calculated by multiplying the BMR by an activity level factor, such as:

- Sedentary (little or no exercise): 1.2
- Lightly active (light exercise/sports 1-3 days/week): 1.375
- Moderately active (moderate exercise/sports 3-5 days/week): 1.55
- Very active (hard exercise/sports 6-7 days/week): 1.725
- Super active (very hard exercise and a physical job): 1.9

Reference: [TDEE Explanation](https://en.wikipedia.org/wiki/Harris–Benedict_equation#Total_daily_energy_expenditure_(TDEE))
