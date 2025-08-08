fn main() {
    // Rust has several data types that are categorized into scalar and compound types
    
    // Scalar Types:
    
    // Integer types (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128)
    let a: i32 = 10; // Signed 32-bit integer
    
    // Floating-point types (f32, f64)
    let b: f64 = 20.5; // 64-bit float, more precise than f32
    
    // Character type (represents a Unicode scalar value)
    let c: char = 'A'; // Uses single quotes for char literals
    
    // Boolean type (true or false)
    let d: bool = true; // Boolean value
    
    // String slice (borrowed string type)
    let e: &str = "Hello, Rust!"; // String literal stored in program memory
    
    // Printing all variables with their respective types
    println!("Integer: {}, Float: {}, Char: {}, Bool: {}, String: {}", a, b, c, d, e);
}
