use std::io;

const DISPLAY_MASS_OPTIONS: &str = "1. Kg
2. Pounds";

const DISPLAY_DISTANCE_OPTIONS: &str = "1. Km
2. Miles";

const DISPLAY_CONVERSION: &str = "1. Mass
2. Distance";

const ASK_FOR_VALUE: &str = "Enter a number";

struct MassConverter {
    value: f32,
    from: String,
    to: String,
}

impl MassConverter {
    fn new(value: f32, from: String, to: String) -> MassConverter {
        MassConverter { value, from, to }
    }

    fn converter(&self) -> Option<f32> {
        if self.from == "Kg" && self.to == "Pound" {
            Some(Self::kg_to_pound(self.value))
        } else {
            None
        }
    }

    fn kg_to_pound(value: f32) -> f32 {
        value * 2.2
    }
}

struct DistanceConverter {
    value: f32,
    from: String,
    to: String,
}

impl DistanceConverter {
    fn new(value: f32, from: String, to: String) -> DistanceConverter {
        DistanceConverter { value, from, to }
    }
}

enum Converter {
    Mass(MassConverter),
    Distance(DistanceConverter),
}

fn mass_conversion() {
    println!("{}", DISPLAY_MASS_OPTIONS);
    let mut mass_choice = String::new();
    io::stdin()
        .read_line(&mut mass_choice)
        .expect("Failed to read line");
    let choice: Vec<&str> = mass_choice.split_whitespace().collect();
    let from = String::from(choice[0]);
    let to = String::from(choice[1]);
    println!("{}", ASK_FOR_VALUE);
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Not an valid number");

    println!("{}, {}, {}", value, from, to);

    let mass_object = MassConverter::new(value, from, to);

    let result = mass_object.converter();

    match result {
        Some(t) => println!("The value of {} after conversion is {}.", value, t),
        None => println!("Will Implement later."),
    }
}

fn distance_conversion() {
    println!("{}", DISPLAY_DISTANCE_OPTIONS);

    let mut distance_choice = String::new();
    io::stdin()
        .read_line(&mut distance_choice)
        .expect("Failed to read line");
}

fn conversion_logic(choice: u32) {
    match choice {
        1 => mass_conversion(),
        2 => distance_conversion(),
        _ => {
            println!("Not the valid choice.")
        }
    }
}

fn main() {
    println!("{}", DISPLAY_CONVERSION);
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Not an integer");
    conversion_logic(choice)
}
