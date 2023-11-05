fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(a / b)
    }
}

use std::io;

fn main() {
    println!("Simple Calculator");
    println!("-----------------");

    // Read the first number
    let mut num1 = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Invalid input");

    // Read the second number
    let mut num2 = String::new();
    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Invalid input");

    // Read the operation
    let mut operation = String::new();
    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    // Perform the calculation
    let result = match operation {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => match divide(num1, num2) {
            Ok(result) => result,
            Err(error) => {
                println!("{}", error);
                return;
            }
        },
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    // Display the result
    println!("Result: {}", result);
}
