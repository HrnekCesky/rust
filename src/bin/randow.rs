fn return_number(number: i32) -> i32 {
    println!("Here is your number.");
    number * 12 // No semicolon: calculates and returns 60
}

fn main() {
    let test = return_number(5);
    println!("{}", test);
}