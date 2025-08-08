fn main() {
    // Control Flow in Rust
    // 1. if-else Statements
    // - Basic conditional execution
    // - No parentheses needed around condition
    // - Condition must be a boolean
    // - Blocks must be enclosed in curly braces

    let number = 10;
    
    // Basic if-else structure
    if number < 0 {
        println!("The number is negative.");
    } else if number == 0 {
        println!("The number is zero.");
    } else {
        println!("The number is positive.");
    }
    
    // If-else as an Expression
    // - In Rust, if-else is an expression
    // - Can be used to assign values
    // - All branches must return same type
    let greeting = if number > 0 {
        "Hello, positive number!"
    } else {
        "Hello, non-positive number!"
    };
    println!("{}", greeting);

    // Match Expression
    // - Pattern matching construct
    // - More powerful alternative to if-else
    // - Must be exhaustive (cover all cases)
    // - _ is a catch-all pattern
    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."), // Default case
    }

    // Match with Multiple Patterns
    let is_weekend = match day {
        6 | 7 => true,    // Multiple values using |
        1..=5 => false,   // Range of values
        _ => false,       // Default case
    };
    println!("Is it weekend? {}", is_weekend);
}
