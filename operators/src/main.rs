fn main() {
    // Arithmetic operators
    let add = 5 + 2; // addition operator
    let subtract = 5 - 2; // substraction operator
    let multiply = 5 * 2; // multiplication operator
    let divide = 5 / 2; // division operator
    let remainder = 5 % 2; // remainder operator
    println!("Addition: {}, Subtraction: {}, Multiplication: {}, Division: {}, Remainder: {}", add, subtract, multiply, divide, remainder);

    // Assignment operators
    let mut x = 5; // mutable variable
    x += 2; // add and assign
    println!("Value of x after addition: {}", x);
    x -= 2; // subtract and assign
    println!("Value of x after subtraction: {}", x);
    x *= 2; // multiply and assign
    println!("Value of x after multiplication: {}", x);
    x /= 2; // divide and assign
    println!("Value of x after division: {}", x);
    x %= 2; // remainder and assign
    println!("Value of x after remainder: {}", x);

    // Comparison operators
    let is_equal = 5 == 2; // equality operator
    let is_not_equal = 5 != 2; // inequality operator
    let is_greater = 5 > 2;   // greater than operator
    let is_less = 5 < 2; // less than operator
    let is_greater_or_equal = 5 >= 2; // greater than or equal to operator
    let is_less_or_equal = 5 <= 2; // less than or equal to operator
    println!("Is Equal: {}, Is Not Equal: {}, Is Greater: {}, Is Less: {}, Is Greater or Equal: {}, Is Less or Equal: {}", is_equal, is_not_equal, is_greater, is_less, is_greater_or_equal, is_less_or_equal);

    // logical operators
    let and = true && false; // logical AND operator
    let or = true || false; // logical OR operator
    let not = !true; // logical NOT operator
    println!("Logical AND: {}, Logical OR: {}, Logical NOT: {}", and, or, not);

    // bitwise operators
    let bitwise_and = 5 & 3; // bitwise AND operator
    let bitwise_or = 5 | 3; // bitwise OR operator
    let bitwise_xor = 5 ^ 3; // bitwise XOR operator
    let bitwise_not = !5; // bitwise NOT operator
    let left_shift = 5 << 1; // left shift operator
    let right_shift = 5 >> 1; // right shift operator
    println!("Bitwise AND: {}, Bitwise OR: {}, Bitwise XOR: {}, Bitwise NOT: {}, Left Shift: {}, Right Shift: {}", bitwise_and, bitwise_or, bitwise_xor, bitwise_not, left_shift, right_shift);
}
