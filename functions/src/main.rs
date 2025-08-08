// This is a simple Rust program that prints "Hello, world!" to the console.
// It demonstrates the basic structure of a Rust program with a main function.
// The function should always be named with lowercase letters.
fn say_hello(){
    println!("Hello, world!");
}

fn temp(num: i32)->i32{
    return num + 1;
}

fn main() {
    say_hello();
    println!("The result is: {}", temp(5));
}
