fn main() {
    // variables are used to store data
    // let is a keyword used to declare a variable
    // variable values cannot be changed by default
    let x = 5; // x is a variable that holds the value 5
    println!("The value of x is: {}", x); // {} is a placeholder for the value of x in the string
    let y = "hello";
    println!("The value of y is: {}", y);

    // we can use mut keyword to make a variable mutable
    let mut z = 10; // z is a mutable variable that holds the value
    z = 10; // variable value changed to 10
    println!("The value of z is: {}", z);
}
