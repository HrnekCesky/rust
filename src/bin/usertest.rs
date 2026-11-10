fn handle_input(user_input: &str) {
    // Attempt to convert the text to an integer
    match user_input.trim().parse::<i32>() {
        Ok(number) => {
            println!("Valid number received: {}", number);
        }
        Err(_) => {
            println!("'{}' is not a valid number. Please try again!", user_input.trim());
        }
    }
}

fn main() {
    handle_input("32");   // Valid number received: 32
    handle_input("a32");  // 'a32' is not a valid number. Please try again!
}