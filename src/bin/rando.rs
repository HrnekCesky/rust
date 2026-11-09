fn return_number(number: i32) -> i32 {
    println!("Here is your number.");
    number * 12
}

fn main() {
    let test = return_number(5);
    println!("{}", test)
}
