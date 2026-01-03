use core::num;
use std::io;

fn main() {

    println!("🧮 Simple Calculator");
    println!("Available operations: +, -, *, /, %, ^");
    println!("Enter your expression (e.g., 3 + 4):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Error: Please enter a valid expression in the format: number operator number");
        return; 
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: '{}' is not a valid number", tokens[0]);
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: '{}' is not a valid number", tokens[2]);
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        "%" => modulo(num1, num2),
        "^" => power(num1, num2),  
        _ => {
            println!("❌ Invalid operator: '{}' Please use one of +, -, *, /", operator);
            return;
        }
    };

    println!("✅ Result: {:.2}", result);
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Division by zero is not allowed");
        std::process::exit(1);
    }
    a / b
}

fn modulo(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Modulo by zero is not allowed");
        std::process::exit(1);
    }
    a % b
}

fn power(a: f64, b: f64) -> f64 {
    a.powf(b)
}