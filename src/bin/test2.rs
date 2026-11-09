struct Person {
    name: String,
    real_name: String,
    f_p_l: String,
    happiness: bool,
}

fn main() {
    let hrnek = Person {
        name: "Hrnek".to_string(),
        real_name: "Adam".to_string(),
        f_p_l: "Python".to_string(),
        happiness: false,
    };

    let Person {
        name: a,
        real_name: b,
        f_p_l: c,
        happiness: d,
    } = hrnek;

    // In Rust, if-else can directly assign the result to a variable:
    let d1 = if !d {
        "No"
    } else {
        "Yes"
    };

    println!(
        "They call him {} but his real name is {}. His favourite programming language is {}, is he happy? {}",
        a, b, c, d1
    );
}
