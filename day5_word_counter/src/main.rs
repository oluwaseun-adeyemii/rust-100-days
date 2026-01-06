use std::env;
use std::fs::File;
use std::io::{Read};

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();

    // check if the correct number of arguments is provided
    if args.len() != 2 {
        println!("❌ Usage: Cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("🔍 Reading file: {}", file_path);

    // Read file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("❌ Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("❌ Error reading file: {}", e);
        return;
    }

    // Count Words
    let word_count = count_words(&contents);
    println!("✅ Word Count: {}", word_count);

    // Count Lines
    let line_count = count_lines(&contents);
    println!("✅ Line Count: {}", line_count);

    // Count Characters 
    let char_count = count_char(&contents);
    println!("✅ Character Count: {}", char_count);
    
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

fn count_char(text: &str) -> usize {
    text.chars().count()
}
        