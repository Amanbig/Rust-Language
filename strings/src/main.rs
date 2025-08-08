fn main() {
    // This is a simple Rust program that prints a greeting message to the console.
    let greeting: &str = "Hello, world!";
    println!("{}", greeting);

    // Demonstrating string operations
    let farewell = "Hello".to_string();
    println!("{}", farewell);

    // Creating a mutable string and performing various operations
    let mut temp = String::from("Hello");
    println!("{}", temp);

    // Appending to the mutable string
    temp.push_str(" world!");
    println!("{}", temp);

    // Appending a single character
    temp.push('!');
    println!("{}", temp);

    // Combining strings
    let s = format!("{} {}", temp, farewell);
    println!("{}", s);

    // Concatenating strings using the + operator
    let s1 = temp+ &farewell;
    println!("{}", s1);

    // Displaying the length and capacity of the string
    println!("Length of s1: {}", s1.len());
    println!("Capacity of s1: {}", s1.capacity());
}
