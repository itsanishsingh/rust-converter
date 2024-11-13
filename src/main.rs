use core::{f32, panic};
use std::io;

const DISPLAY_MASS_OPTIONS: &str = "1. Kg
2. Pound
3. Gram";

const DISPLAY_DISTANCE_OPTIONS: &str = "1. Km
2. Miles";

const DISPLAY_CONVERSION: &str = "1. Mass
2. Distance";

const ASK_FOR_UNITS: &str = "Enter the name of units, divided with a space";

const ASK_FOR_VALUE: &str = "Enter a number";

enum MassOptions {
    Kg,
    Pound,
    Gram,
}

struct MassConverter {
    value: f32,
    from: MassOptions,
    to: MassOptions,
}

impl MassConverter {
    fn new(value: f32, from: MassOptions, to: MassOptions) -> MassConverter {
        MassConverter { value, from, to }
    }

    fn converter(&self) -> f32 {
        match self.from {
            MassOptions::Kg => match self.to {
                MassOptions::Kg => self.value,
                MassOptions::Pound => Self::kg_to_pound(self.value),
                MassOptions::Gram => Self::kg_to_g(self.value),
            },
            MassOptions::Pound => match self.to {
                MassOptions::Kg => Self::pound_to_kg(self.value),
                MassOptions::Pound => self.value,
                MassOptions::Gram => Self::pound_to_g(self.value),
            },
            MassOptions::Gram => match self.to {
                MassOptions::Kg => Self::g_to_kg(self.value),
                MassOptions::Pound => Self::g_to_pound(self.value),
                MassOptions::Gram => self.value,
            },
        }
    }

    fn kg_to_pound(value: f32) -> f32 {
        value * 2.2
    }

    fn kg_to_g(value: f32) -> f32 {
        value * 1000_f32
    }

    fn pound_to_kg(value: f32) -> f32 {
        value / 2.2
    }

    fn pound_to_g(value: f32) -> f32 {
        value / 2.2 * 1000.0
    }

    fn g_to_kg(value: f32) -> f32 {
        value / 1000_f32
    }

    fn g_to_pound(value: f32) -> f32 {
        value / 1000.0 * 2.2
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

    fn converter(&self) -> f32 {
        if self.from == "Kg" && self.to == "Pound" {
            Self::km_to_mile(self.value)
        } else {
            0.0
        }
    }

    fn km_to_mile(value: f32) -> f32 {
        value / 1.6
    }
}

enum Converter {
    Mass(MassConverter),
    Distance(DistanceConverter),
}

impl Converter {
    fn convert(&self) -> f32 {
        match self {
            Self::Mass(m) => m.converter(),
            Self::Distance(d) => d.converter(),
        }
    }
}

fn mass_conversion() {
    println!("{}", DISPLAY_MASS_OPTIONS);

    println!("{}", ASK_FOR_UNITS);
    let mut mass_choice = String::new();
    io::stdin()
        .read_line(&mut mass_choice)
        .expect("Failed to read line");

    let choice: Vec<&str> = mass_choice.split_whitespace().collect();
    let from = String::from(choice[0]).to_lowercase();
    let to = String::from(choice[1]).to_lowercase();

    println!("{}", ASK_FOR_VALUE);
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Not an valid number");

    let kg = String::from("kg");
    let pound = String::from("pound");
    let g = String::from("gram");

    let from: MassOptions = if from == kg {
        MassOptions::Kg
    } else if from == pound {
        MassOptions::Pound
    } else if from == g {
        MassOptions::Gram
    } else {
        panic!()
    };

    let to: MassOptions = if to == kg {
        MassOptions::Kg
    } else if to == pound {
        MassOptions::Pound
    } else if to == g {
        MassOptions::Gram
    } else {
        panic!()
    };

    let mass_object = Converter::Mass(MassConverter::new(value, from, to));

    let result = mass_object.convert();

    println!("The value of {} after conversion is {}.", value, result);
}

fn distance_conversion() {
    println!("{}", DISPLAY_DISTANCE_OPTIONS);

    println!("{}", ASK_FOR_UNITS);
    let mut distance_choice = String::new();
    io::stdin()
        .read_line(&mut distance_choice)
        .expect("Failed to read line");

    let choice: Vec<&str> = distance_choice.split_whitespace().collect();
    let from = String::from(choice[0]).to_lowercase();
    let to = String::from(choice[1]).to_lowercase();
    println!("{}", ASK_FOR_VALUE);
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    let value: f32 = value.trim().parse().expect("Not an valid number");

    println!("{}, {}, {}", value, from, to);

    let distance_object = Converter::Distance(DistanceConverter::new(value, from, to));

    let result = distance_object.convert();

    println!("The value of {} after conversion is {}.", value, result);
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
