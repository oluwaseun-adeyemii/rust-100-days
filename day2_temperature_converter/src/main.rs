use std::io;

fn main() {
    println!("🌡️ Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("3: Kelvin to Celsius");
    println!("4: Celsius to Kelvin");
    println!("5: Kelvin to Fahrenheit");
    println!("6: Fahrenheit to Kelvin");
    println!("Please select an option (1 through 6):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 through 6.");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else if choice == 3 {
        kelvin_to_celsius();
    } else if choice == 4 {
        celsius_to_kelvin();
    } else if choice == 5 {
        kelvin_to_fahrenheit();
    } else if choice == 6 {
        fahrenheit_to_kelvin();
    } else {
        println!("❌ Invalid choice. Please enter 1 through 6.");
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("🌡️ {:.2}°C is equal to {:.2}°F", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("🌡️ {:.2}°F is equal to {:.2}°C", temp, celsius);
}

fn kelvin_to_celsius() {
    println!("Enter temperature in Kelvin:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let celsius = temp - 273.15;
    println!("🌡️ {:.2}K is equal to {:.2}°C", temp, celsius);
}

fn celsius_to_kelvin() {
    println!("Enter temperature in Celsius:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let kelvin = temp + 273.15;
    println!("🌡️ {:.2}°C is equal to {:.2}K", temp, kelvin);
}

fn kelvin_to_fahrenheit() {
    println!("Enter temperature in Kelvin:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (temp - 273.15) * 9.0 / 5.0 + 32.0;
    println!("🌡️ {:.2}K is equal to {:.2}°F", temp, fahrenheit);
}

fn fahrenheit_to_kelvin() {
    println!("Please Enter temperature in Fahrenheit:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let kelvin = (temp - 32.0) * 5.0 / 9.0 + 273.15;
    println!("🌡️ {:.2}°F is equal to {:.2}K", temp, kelvin);
}

