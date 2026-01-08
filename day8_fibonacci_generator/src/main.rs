use core::num;
use std::io;

fn main() {
    println!("🔢 Fibonacci Sequence Generator");
    println!("Enter the number of terms you want to generate:");

    let num_terms = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("❌ Invalid input. Please enter a positive integer.");
            return;
        }
    };

    if num_terms == 0 {
        println!("❌ Please enter a number greater than zero.");
        return;
    }   

    let sequence = generate_fibonacci(num_terms);
    println!("✅ Fibonacci sequence with {} terms: {:?}", num_terms, sequence);
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
    
    if n >= 1 {
        sequence.push(0);
    }

    if n >= 2 {
        sequence.push(1);
    }

    for i in 2..n {
        let next_value = sequence[(i - 1) as usize] + sequence[(i - 2) as usize];
        sequence.push(next_value);
    }

    sequence

}

