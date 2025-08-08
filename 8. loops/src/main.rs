fn main() {
    // This program demonstrates different types of loops in Rust.
    let mut count = 0;
    // Infinite loop using `loop`
    loop{
        count += 1;
        if count == 5{
            break;
        }
        println!("Hello,world!");
    }
    println!("Loop ended after {} iterations", count);

    // Using `loop` with a return value
    let result = loop {
        count += 1;
        if count == 10{
            break count;
        }
    };
    println!("Count reached: {}", result);

    // Using `while` loop
    let mut n = 1;
    while n <= 5 {
        println!("n is {}",n);
        n += 1;
    }

    // Using `for` loop with a range
    for i in 1..5 {
        println!("i is {}",i);
    }
    
    // Using `for` loop with an inclusive range
    for i in 1..=5 {
        println!("i is {}",i);
    }
}
