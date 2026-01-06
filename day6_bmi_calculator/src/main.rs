// BMI = weight / (height * height)
// BMI Categories:
// Underweight = <18.5
// Normal weight = 18.5–24.9
// Overweight = 25–29.9
// Obesity = BMI of 30 or greater

use std::io;

fn main() {
    println!("⚖️ BMI Calculator ⚖️");
    println!("Please enter your weight in kilograms (KG):");

    let weight = match get_input_asf64() {
        Some(value) => value,
        None => {
            println!("❌ Invalid input for weight. Please enter a valid number.");
            return;
        }
    };

    println!("Please enter your height in meters (M):");
    let height = match get_input_asf64() {
        Some(value) => value,
        None => {
            println!("❌ Invalid input for height. Please enter a valid number.");
            return;
        }
    };

    if height == 0.0 {
        println!("❌ Height cannot be zero.");
        return;
    }

    let bmi = calculate_bmi(weight, height);
    let category = categorize_bmi(bmi);
    println!("Your BMI is: {bmi:.2}");
    println!("You are categorized as: {category}");

    println!("Thank you for using the BMI Calculator! Stay healthy! 🌟");

}

fn get_input_asf64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    match input.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn categorize_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal weight"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    }
}