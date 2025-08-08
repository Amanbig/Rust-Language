// Functions in Rust
// 1. Functions are declared using the 'fn' keyword
// 2. Function names use snake_case convention
// 3. Functions can take parameters and return values
// 4. The main function is the entry point of the program

// A simple function with no parameters and no return value
fn say_hello() {
    println!("Hello, world!");
}

// A function that:
// - Takes a parameter 'num' of type i32 (32-bit integer)
// - Returns an i32 value (specified after ->)
// - Adds 1 to the input number and returns it
fn increment(num: i32) -> i32 {
    // In Rust, the last expression in a block is implicitly returned
    // No need for 'return' keyword if it's the last expression
    num + 1
}

fn main() {
    // Calling a function with no parameters
    say_hello();
    
    // Calling a function with a parameter and printing its return value
    let result = increment(5);
    println!("5 incremented by 1 is: {}", result);
    
    // We can also use the function call directly in println!
    println!("10 incremented by 1 is: {}", increment(10));
}
