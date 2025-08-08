fn main() {
    // Constants in Rust
    // 1. Constants are values that cannot be changed throughout the program
    // 2. Must be declared with the 'const' keyword
    // 3. Type annotation is mandatory (unlike variables)
    // 4. Must be assigned a constant expression (computed at compile time)
    // 5. By convention, constants are named in SCREAMING_SNAKE_CASE
    // 6. Constants can be declared in any scope, including global scope
    
    const PI: f64 = 3.14159265359;
    const MAX_POINTS: u32 = 100_000;
    const STARTING_LEVEL: i32 = 1;
    
    println!("Mathematical constant PI: {}", PI);
    println!("Maximum points possible: {}", MAX_POINTS);
    println!("Starting level: {}", STARTING_LEVEL);
    
    // Constants vs Variables:
    // - Constants are always immutable (no 'mut' keyword allowed)
    // - Type must be annotated
    // - Value must be known at compile time
    // - Can be declared in any scope
}
