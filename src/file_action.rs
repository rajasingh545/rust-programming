use std::fs::File;
use std::io::{self, Write};

/// This function gets user input for the file name and content.
/// It prompts the user for the file name and content to write.
/// # Returns
/// A tuple containing the file name and content.
/// # Errors
/// This function will return an error if the input cannot be read.
/// # Panics
/// This function will panic if the input cannot be read.
/// # Safety
/// This function is not unsafe, but it will panic if the input cannot be read.
/// # Examples
/// ```
/// let (file, content) = get_user_input();
/// assert_eq!(file, "example.txt");
/// assert_eq!(content, "Hello, world!");
/// ```
/// # Note
/// This function will return an error if the input cannot be read.
/// # Warning
/// This function will panic if the input cannot be read.
/// # Example
/// ```
/// let (file, content) = get_user_input();
/// assert_eq!(file, "example.txt");
/// assert_eq!(content, "Hello, world!");
/// ```
pub fn get_user_input() -> io::Result<(String, String)> {
    let input = io::stdin();

    println!("What file would you like to write?");
    let mut requested_file = String::new();
    input.read_line(&mut requested_file)?;

    println!("What content would you like to write?");
    let mut content = String::new();
    input.read_line(&mut content)?;

    return Ok((
        requested_file.trim().to_string(),
        content.trim().to_string(),
    ));
}

/// Writes the content to the specified file.
/// If the file already exists, it will be overwritten.
/// If the file cannot be created, an error will be returned.
/// # Arguments
/// * `path` - A string slice that holds the path to the file.
/// * `content` - A string slice that holds the content to write to the file.
/// # Example
/// ```
/// let path = "example.txt";
/// let content = "Hello, world!";
/// write_to_file(path, content);
/// ```
/// # Errors
/// This function will return an error if the file cannot be created or written to.
/// # Panics
/// This function will panic if the file cannot be created or written to.
/// # Safety
/// This function is not unsafe, but it will panic if the file cannot be created or written to.
/// # Examples
/// ```
/// let path = "example.txt";
/// let content = "Hello, world!";
/// write_to_file(path, content);
/// ```
/// # Note
/// This function will overwrite the file if it already exists.
/// # Warning
/// This function will panic if the file cannot be created or written to.
pub fn write_to_file(path: &str, content: &str) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}

fn remove_file(path: &str) {
    if let Err(e) = std::fs::remove_file(path) {
        eprintln!("Error removing file: {}", e);
    } else {
        println!("File removed successfully");
    }
}
