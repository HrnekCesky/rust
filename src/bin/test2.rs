struct Person { // make a simple struct for a person
    name: String,
    real_name: String,
    f_p_l: String,
    happiness: bool
}

fn main() {
    let papa_doc = Person { // create variable papa_doc
        name: "Hrnek".to_string(),
        real_name: "Adam".to_string(),
        f_p_l: "Python".to_string(),
        happiness: false
    };

    let Person { // destructure papa_doc
        name: a,
        real_name: b,
        f_p_l: c,
        happiness: d
    } = papa_doc;

    println!("They call him {} but his real name is {}. His favourite programing language is {}, is he happy? {}", a, b, c, d);
}

