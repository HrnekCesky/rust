use std::io::{self, Write};
1
fn main() {
    println!("===========================");
    println!("  Rust CLI Calculator");
    println!("===========================");

    loop {
        println!();
        println!("Choose an operation:");
        println!("  1) Add");
        println!("  2) Subtract");
        println!("  3) Multiply");
        println!("  4) Divide");
        println!("  5) Power");
        println!("  6) Quit");
        print!("> ");
        io::stdout().flush().unwrap();

        let choice = read_line_trimmed();

        match choice.as_str() {
            "6" | "q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "1" | "2" | "3" | "4" | "5" => {
                let op_name = match choice.as_str() {
                    "1" => "Addition",
                    "2" => "Subtraction",
                    "3" => "Multiplication",
                    "4" => "Division",
                    "5" => "Power",
                    _ => unreachable!(),
                };
                println!("\n--- {op_name} ---");

                let a = match read_number("Enter the first number: ") {
                    Some(n) => n,
                    None => continue,
                };
                let b = match read_number("Enter the second number: ") {
                    Some(n) => n,
                    None => continue,
                };

                let result = match choice.as_str() {
                    "1" => Some(a + b),
                    "2" => Some(a - b),
                    "3" => Some(a * b),
                    "4" => {
                        if b == 0.0 {
                            println!("Error: division by zero!");
                            None
                        } else {
                            Some(a / b)
                        }
                    }
                    "5" => Some(a.powf(b)),
                    _ => unreachable!(),
                };

                if let Some(r) = result {
                    println!("Result: {r}");
                }
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

/// Reads a line from stdin and returns it trimmed.
fn read_line_trimmed() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    buf.trim().to_string()
}

/// Prompts the user for a number, returning `None` on invalid input.
fn read_number(prompt: &str) -> Option<f64> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let input = read_line_trimmed();
    match input.parse::<f64>() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("Invalid number: \"{input}\"");
            None
        }
    }
}
