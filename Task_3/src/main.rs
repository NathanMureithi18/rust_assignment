use std::io;

fn main() {
    // Function to read a number from the terminal
    fn read_number(prompt: &str) -> f64 {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please enter a valid number")
    }

    // Read two numbers from the user
    let num1 = read_number("Enter the first number:");
    let num2 = read_number("Enter the second number:");

    // Perform arithmetic operations
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let division = if num2 != 0.0 {
        num1 / num2
    } else {
        f64::NAN
    };

    // results
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    if num2 != 0.0 {
        println!("Division: {}", division );
    } else {
        println!("Division: undefined (division by zero)");
    }
}
