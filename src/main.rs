mod converter;
use converter::*;
use std::io;

fn main() {
    println!("Hello, world!");

    // Get inputs
    let from_choice = input("Convert from (C/F): ".to_string()).unwrap();
    let to_choice = input("Convert to (C/F): ".to_string()).unwrap();
    let value = input("Enter the value: ".to_string()).unwrap().parse::<f64>().expect("Invalid number!");

    // Convertr the value to celcius
    let celcius_value = match from_choice.as_str() {
        "C" => value,
        "F" => farenheight_to_celcius(value),
        _ => {
            println!("Invalid choice to convert from");
            return;
        }
    };

    let output_value = match to_choice.as_str() {
        "C" => celcius_value,
        "F" => celcius_to_farenheight(celcius_value),
        _ => {
            println!("Invalid choice to convert to");
            return;
        }
    };

    println!("The converted value is: {}", output_value);
}

fn input(question: String) -> io::Result<String> {
    use std::io::Write;

    println!("{}", question); // Print the question
    io::stdout().flush()?; // Flush the stdout buffer

    let mut buffer = String::new(); // Create a new mutable string
    io::stdin().read_line(&mut buffer)?; // Read the input from the user
    Ok(buffer.trim().to_string()) // Return the trimmed string
}