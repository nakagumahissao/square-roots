//! # Babylonian Square Root Utility
//! 
//! This crate provides a CLI tool to approximate square roots using the 
//! Babylonian method (Heron's method). It demonstrates iterative 
//! algorithms, ANSI terminal control, and user input handling in Rust.

/// Calculates the square root of a number using the Babylonian method.
///
/// The method refines a guess using the formula: 
/// $guess = guess + \frac{num - guess^2}{2 \times guess}$
///
/// # Arguments
/// * `num` - The value to calculate the square root of.
/// * `iterations` - The maximum number of iterations allowed.
///
/// # Returns
/// A tuple containing `(approximation, actual_iterations_performed)`.
///
/// # Examples
/// ```
/// let (result, iters) = evaluate(25.0, 10);
/// assert!((result - 5.0).abs() < f64::EPSILON);
/// ```
fn evaluate(num: f64, iterations: i32) -> (f64, i32) {
    if num == 0.0 { return (0.0, 0); }

    let mut guess = num / 2.0;
    let epsilon = f64::EPSILON; 
    let mut last_iter = 0;

    for i in 0..iterations {
        let last_guess = guess;
        
        // guess = guess + (num - guess^2) / (2 * guess)
        guess = guess + ((num - (guess * guess)) / (2.0 * guess));
        last_iter = i + 1;

        // Convergence check: stop if the change is negligible
        if (last_guess - guess).abs() <= epsilon * guess.abs() {
            break;
        }
    }

    (guess, last_iter)
}

/// Clears the terminal screen using ANSI escape sequences.
///
/// Note: This may not work in all IDE consoles (like the default VS Code output),
/// but works in most standard terminal emulators.
fn clearscreen() {
    print!("\x1B[2J\x1B[1;1H");
}   

/// Prompts the user for input and attempts to parse it as an `f64`.
///
/// # Panics
/// Panics if the input cannot be parsed into a valid 64-bit float.
fn askinputnumber(prompt: &str) -> f64 {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().parse::<f64>().expect("Please enter a valid number")
}   

fn main() {
    clearscreen();

    println!("\n--- Babylonian Square Root Calculator ---");
    let number = askinputnumber("Enter a number: ");
    let iterations = askinputnumber("Enter max iterations: ") as i32;

    let (val, iters) = evaluate(number, iterations);
    let correct = number.sqrt();

    println!("\nRESULTS:");
    println!("Approximation: {:.10}", val);
    println!("Iterations:    {}", iters);
    println!("Standard Lib:  {:.10}", correct);
    println!("Difference:    {:.10}", (val - correct).abs());
}