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

    // example of ownership
    // In Rust, ownership is a key concept that ensures memory safety.
    // Here, we demonstrate ownership by transferring ownership of a string.
    // When `a` is assigned to `b`, `b` takes ownership of the string,
    // and `a` can no longer be used to access it.
    // But for intergers, ownership is not transferred.
    // Instead, a copy is made.
    let a = String::from("Hello, world!");
    let b = a;
    println!("{}", b);

    let a = String::from("Hello");
    let b = a.clone(); // Now both have the same value

    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works

    let x = String::from("hello");
    let y = &x; // y is a reference to x
    println!("x = {}, y = {}", x, y); // Both can be used
}
