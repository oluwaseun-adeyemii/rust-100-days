use std::io;

fn main() {
    println!("🔢 Prime Number Checker");
    println!("Please enter a number to check if it's prime:");

    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input. Please enter a valid positive integer.");
            return;
        }
    };

    if number <= 1 {
        println!("❌ The number {} is not prime.", number);
        return;
    }
    
    if is_prime(number) {
        println!("✅ The number {} is prime!", number);
    } else {
        println!("❌ The number {} is not prime.", number);
    }
}

/// Reads user input and attempts to parse it as a u32.

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

/// Checks if a number is prime.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}