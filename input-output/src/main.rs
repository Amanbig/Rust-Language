use std::io::{self, Write, Read};

// Function to demonstrate basic string input
fn get_string_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    // Flush ensures the prompt is displayed before waiting for input
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Trim the newline character from the end
    Ok(input.trim().to_string())
}

// Function to demonstrate number input with error handling
fn get_number_input(prompt: &str) -> io::Result<i32> {
    let input = get_string_input(prompt)?;
    match input.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => {
            eprintln!("Error: Please enter a valid number");
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a number"))
        }
    }
}

// Function to demonstrate multiple input values
fn get_multiple_values() -> io::Result<()> {
    println!("Enter three numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    println!("Parsed numbers: {:?}", numbers);
    Ok(())
}

// Function to demonstrate continuous input until a condition
fn interactive_loop() -> io::Result<()> {
    loop {
        let input = get_string_input("Enter a command (quit to exit): ")?;
        
        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" => println!("Available commands: help, quit, echo <text>"),
            _ if input.starts_with("echo ") => {
                println!("{}", &input["echo ".len()..]);
            }
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }
    Ok(())
}

fn main() {
    println!("=== Rust Input/Output Examples ===\n");

    // Example 1: Basic string input
    match get_string_input("Enter your name: ") {
        Ok(name) => println!("Hello, {}!\n", name),
        Err(e) => eprintln!("Error reading name: {}\n", e),
    }

    // Example 2: Number input with error handling
    match get_number_input("Enter your age: ") {
        Ok(age) => println!("In 10 years, you'll be {} years old!\n", age + 10),
        Err(e) => eprintln!("Error reading age: {}\n", e),
    }

    // Example 3: Multiple values input
    if let Err(e) = get_multiple_values() {
        eprintln!("Error reading multiple values: {}\n", e);
    }

    // Example 4: Different types of output
    // Standard output
    println!("This is a normal message"); // Adds newline
    print!("This is without newline");    // No newline
    print!(" (continued)\n");             // Manual newline
    
    // Error output
    eprintln!("This is an error message"); // Prints to stderr
    
    // Formatted output
    println!("Formatted: {:>10}", "right"); // Right-aligned in 10 spaces
    println!("Decimal: {:.2}", 3.14159);    // 2 decimal places
    println!("Binary: {:b}, Hex: {:x}", 42, 42); // Different number formats
    
    // Example 5: Interactive loop
    println!("\n=== Interactive Command Loop ===");
    if let Err(e) = interactive_loop() {
        eprintln!("Error in interactive loop: {}", e);
    }
}