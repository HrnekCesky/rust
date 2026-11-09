use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>, // Key: Year (u32), Value: Population (u32)
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(), // Starts empty
    };

    // Insert data with .insert(key, value)
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!("In the year {} the city of Tallinn had a population of {}.", year, population);
    }
}