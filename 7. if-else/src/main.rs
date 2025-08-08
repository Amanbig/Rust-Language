fn main() {
    // if-else statements are used for conditional execution
    let number = 10;
    if number < 0 { // check if the number is negative
        println!("The number is negative.");
    } else if number == 0 { // check if the number is zero
        println!("The number is zero.");
    } else { // if the number is positive
        println!("The number is positive.");
    }
    
    
    let greeting = if number > 0 {
        "Hello, positive number!"
    } else {
        "Hello, non-positive number!"
    };
    println!("{}", greeting); // print the greeting based on the condition

    // match statements can also be used for conditional execution
    let day = 3; // let's say we have a variable day
    match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }

}
