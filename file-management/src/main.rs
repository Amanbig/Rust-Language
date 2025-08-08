// Standard library imports for file operations
use std::fs;  // For high-level file system operations
use std::io::{self, Write, Read};  // For low-level I/O operations
use std::path::Path;  // For path manipulation

// Function to demonstrate file writing operations
fn write_file_example(file_path: &str, content: &str) -> io::Result<()> {
    // Method 1: Simple write using fs::write
    // This is the simplest way to write to a file, good for one-time writes
    fs::write(file_path, content)?;
    println!("File written using fs::write");

    // Method 2: Using File::create and write_all
    // This gives more control over the file handling
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("File written using File::create");

    Ok(())
}

// Function to demonstrate file reading operations
fn read_file_example(file_path: &str) -> io::Result<()> {
    // Method 1: Reading entire file as bytes
    let data_bytes = fs::read(file_path)?;
    println!("Data as bytes: {:?}", data_bytes);

    // Method 2: Reading entire file as string
    let data_string = fs::read_to_string(file_path)?;
    println!("Data as string: {:?}", data_string);

    // Method 3: Reading file using buffer
    // This method is more memory efficient for large files
    let mut file = fs::File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("File content using buffer:\n{}", buffer);

    Ok(())
}

// Function to demonstrate file management operations
fn file_management_example(file_path: &str) -> io::Result<()> {
    // Check if file exists
    if Path::new(file_path).exists() {
        println!("File exists before operations");
    }

    // Create directory if it doesn't exist
    let dir_path = "example_dir";
    fs::create_dir_all(dir_path)?;
    println!("Directory created: {}", dir_path);

    // Copy file
    let backup_path = format!("{}/backup_{}", dir_path, file_path);
    fs::copy(file_path, &backup_path)?;
    println!("File backed up to: {}", backup_path);

    // Remove files and directory
    fs::remove_file(file_path)?;
    fs::remove_file(&backup_path)?;
    fs::remove_dir(dir_path)?;
    println!("Cleanup completed");

    Ok(())
}

fn main() {
    // Define test content and file path
    let content = "This is a sample file content for file management in Rust.";
    let file_path = "sample_file.txt";

    // Error handling with match expression
    match write_file_example(file_path, content) {
        Ok(_) => println!("File writing operations completed successfully"),
        Err(e) => eprintln!("Error during file writing: {}", e),
    }

    match read_file_example(file_path) {
        Ok(_) => println!("File reading operations completed successfully"),
        Err(e) => eprintln!("Error during file reading: {}", e),
    }

    match file_management_example(file_path) {
        Ok(_) => println!("File management operations completed successfully"),
        Err(e) => eprintln!("Error during file management: {}", e),
    }
}
