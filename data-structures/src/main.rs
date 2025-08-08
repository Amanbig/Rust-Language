use std::collections::HashMap;

fn main() {
    // This is a simple Rust program that demonstrates various data structures.
    // It includes arrays, vectors, tuples, hash maps, structs, and enums.
    let a = [1,2,3,4];
    println!("Array: {:?}",a);
    println!("Length of array: {}", a.len());
    println!("First element: {}", a[0]);
    for i in a {
        println!("Element: {}", i);
    }

    // Demonstrating vector operations
    let mut b = vec![1,2,3];
    println!("Vector: {:?}", b);
    b.push(4);
    println!("Vector after push: {:?}", b);
    b.pop();
    println!("Vector after pop: {:?}", b);
    b.insert(0, 0);
    println!("Vector after insert: {:?}", b);
    b.remove(0);
    println!("Vector after remove: {:?}", b);
    println!("Length of vector: {}", b.len());
    println!("Capacity of vector: {}", b.capacity());
    for i in &b {
        println!("Element: {}", i);
    }

    // Demonstrating tuples and hash maps
    // Tuples can hold different types of values, while hash maps store key-value pairs.
    let person = ("Alice", 30, "Engineer");
    println!("Tuple: {:?}", person);
    println!("Name: {}, Age: {}, Occupation: {}", person.0, person.1, person.2);

    // Hash maps are used to store key-value pairs.
    // They are useful for fast lookups and can store values of the same type.
    // Here we create a hash map to store a person's details.
    // We can insert, retrieve, and iterate over the key-value pairs.
    // Note: HashMap requires the `std::collections::HashMap` module.
    let mut map = HashMap::new();
    map.insert("name", "Alice");
    map.insert("age", "30");
    map.insert("occupation", "Engineer");
    println!("HashMap: {:?}", map);
    println!("Name: {}", map.get("name").unwrap());
    println!("Age: {}", map.get("age").unwrap());
    println!("Occupation: {}", map.get("occupation").unwrap());
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    // Demonstrating structs and enums
    // Structs are used to create custom data types with named fields.
    // Enums are used to define a type that can be one of several variants.
    struct Person{
        name: String,
        age: u32,
        occupation: String,
    }
    let user = Person {
        name: String::from("Alice"),
        age: 30,
        occupation: String::from("Engineer"),
    };
    println!("Struct: Name: {}, Age: {}, Occupation: {}", user.name, user.age, user.occupation);


    enum Color {
        Red,
        Green,
        Blue,
    }
    let color = Color::Red;
    println!("Enum: {:?}", color);

}
