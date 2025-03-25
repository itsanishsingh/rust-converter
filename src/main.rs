use std::io;

const ASK_FOR_TYPE: &str = "Select the number for one of the following conversion types:
1. Temperature
2. Distance
3. Weight";

const ASK_FOR_TEMPERATURE: &str = "Select the number for one of the following temperature types:
1. Celcius
2. Kelvin
3. Fahrenheit";

const ASK_FOR_DISTANCE: &str = "Select the number for one of the following distance types:
1. Km
2. Meter
3. Mile";

const ASK_FOR_WEIGHT: &str = "Select the number for one of the following weight types:
1. Kg
2. Gram
3. Pound";

const ASK_FOR_VALUE: &str = "Enter the value which you want to convert:";

#[derive(Debug)]
struct InputError;

enum ConversionOptions {
    Temperature,
    Distance,
    Weight,
}

impl ConversionOptions {
    fn from_str(s: &str) -> Result<ConversionOptions, InputError> {
        match s {
            "1" => Ok(ConversionOptions::Temperature),
            "2" => Ok(ConversionOptions::Distance),
            "3" => Ok(ConversionOptions::Weight),
            _ => Err(InputError),
        }
    }
}

enum TemperatureOptions {
    Celcius,
    Kelvin,
    Fahrenheit,
}

impl TemperatureOptions {
    fn from_str(s: &str) -> Result<TemperatureOptions, InputError> {
        match s {
            "1" => Ok(TemperatureOptions::Celcius),
            "2" => Ok(TemperatureOptions::Kelvin),
            "3" => Ok(TemperatureOptions::Fahrenheit),
            _ => Err(InputError),
        }
    }
}

enum DistanceOptions {
    Km,
    Meter,
    Mile,
}

impl DistanceOptions {
    fn from_str(s: &str) -> Result<DistanceOptions, InputError> {
        match s {
            "1" => Ok(DistanceOptions::Km),
            "2" => Ok(DistanceOptions::Meter),
            "3" => Ok(DistanceOptions::Mile),
            _ => Err(InputError),
        }
    }
}

enum WeightOptions {
    Kg,
    Gram,
    Pound,
}

impl WeightOptions {
    fn from_str(s: &str) -> Result<WeightOptions, InputError> {
        match s {
            "1" => Ok(WeightOptions::Kg),
            "2" => Ok(WeightOptions::Gram),
            "3" => Ok(WeightOptions::Pound),
            _ => Err(InputError),
        }
    }
}

struct TemperatureStruct {
    from: TemperatureOptions,
    to: TemperatureOptions,
    value: f32,
}

impl TemperatureStruct {
    fn new(from: TemperatureOptions, to: TemperatureOptions, value: f32) -> TemperatureStruct {
        TemperatureStruct { from, to, value }
    }

    fn convert(&self) -> f32 {
        match self.from {
            TemperatureOptions::Celcius => match self.to {
                TemperatureOptions::Celcius => self.value,
                TemperatureOptions::Kelvin => self.value,
                TemperatureOptions::Fahrenheit => self.value,
            },
            TemperatureOptions::Kelvin => match self.to {
                TemperatureOptions::Celcius => self.value,
                TemperatureOptions::Kelvin => self.value,
                TemperatureOptions::Fahrenheit => self.value,
            },
            TemperatureOptions::Fahrenheit => match self.to {
                TemperatureOptions::Celcius => self.value,
                TemperatureOptions::Kelvin => self.value,
                TemperatureOptions::Fahrenheit => self.value,
            },
        }
    }
}

struct DistanceStruct {
    from: DistanceOptions,
    to: DistanceOptions,
    value: f32,
}

impl DistanceStruct {
    fn new(from: DistanceOptions, to: DistanceOptions, value: f32) -> DistanceStruct {
        DistanceStruct { from, to, value }
    }

    fn convert(&self) -> f32 {
        match self.from {
            DistanceOptions::Km => match self.to {
                DistanceOptions::Km => self.value,
                DistanceOptions::Meter => self.value * 1000 as f32,
                DistanceOptions::Mile => self.value / 1.6,
            },
            DistanceOptions::Meter => match self.to {
                DistanceOptions::Km => self.value / 1000 as f32,
                DistanceOptions::Meter => self.value / 1600 as f32,
                DistanceOptions::Mile => self.value,
            },
            DistanceOptions::Mile => match self.to {
                DistanceOptions::Km => self.value * 1.6,
                DistanceOptions::Meter => self.value * 1600 as f32,
                DistanceOptions::Mile => self.value,
            },
        }
    }
}

struct WeightStruct {
    from: WeightOptions,
    to: WeightOptions,
    value: f32,
}

impl WeightStruct {
    fn new(from: WeightOptions, to: WeightOptions, value: f32) -> WeightStruct {
        WeightStruct { from, to, value }
    }

    fn convert(&self) -> f32 {
        match self.from {
            WeightOptions::Kg => match self.to {
                WeightOptions::Kg => self.value,
                WeightOptions::Gram => self.value * 1000 as f32,
                WeightOptions::Pound => self.value * 2.2,
            },
            WeightOptions::Gram => match self.to {
                WeightOptions::Kg => self.value / 1000 as f32,
                WeightOptions::Gram => self.value,
                WeightOptions::Pound => self.value / 453.5,
            },
            WeightOptions::Pound => match self.to {
                WeightOptions::Kg => self.value / 2.2,
                WeightOptions::Gram => self.value * 453.5,
                WeightOptions::Pound => self.value,
            },
        }
    }
}

fn main() {
    println!("{}", ASK_FOR_TYPE);
    let mut conversion_type = String::new();

    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Reading input failed.");

    let conversion_type = ConversionOptions::from_str(&conversion_type.trim()).unwrap();
    conversion_process(conversion_type);
}

fn conversion_process(conversion_type: ConversionOptions) {
    match conversion_type {
        ConversionOptions::Temperature => {
            println!("{}", ASK_FOR_TEMPERATURE);
            let (from_org, to_org, value) = get_all_values();
            let from = TemperatureOptions::from_str(from_org.trim()).unwrap();
            let to = TemperatureOptions::from_str(to_org.trim()).unwrap();
            let temp_struct = TemperatureStruct::new(from, to, value);
            let converted_value = temp_struct.convert();
            println!(
                "The value {} from {} to {} is {}.",
                value, from_org, to_org, converted_value
            );
        }
        ConversionOptions::Distance => {
            println!("{}", ASK_FOR_DISTANCE);
            let (from_org, to_org, value) = get_all_values();
            let from = DistanceOptions::from_str(from_org.trim()).unwrap();
            let to = DistanceOptions::from_str(to_org.trim()).unwrap();
            let dist_struct = DistanceStruct::new(from, to, value);
            let converted_value = dist_struct.convert();
            println!(
                "The value {} from {} to {} is {}.",
                value, from_org, to_org, converted_value
            );
        }
        ConversionOptions::Weight => {
            println!("{}", ASK_FOR_WEIGHT);
            let (from_org, to_org, value) = get_all_values();
            let from = WeightOptions::from_str(from_org.trim()).unwrap();
            let to = WeightOptions::from_str(to_org.trim()).unwrap();
            let weight_struct = WeightStruct::new(from, to, value);
            let converted_value = weight_struct.convert();
            println!(
                "The value {} from {} to {} is {}.",
                value, from_org, to_org, converted_value
            );
        }
    }
}

fn get_all_values() -> (String, String, f32) {
    let mut from = String::new();
    let mut to = String::new();
    let mut value = String::new();

    println!("Enter number for from:");
    io::stdin()
        .read_line(&mut from)
        .expect("Reading input failed");

    println!("Enter number for to:");
    io::stdin()
        .read_line(&mut to)
        .expect("Reading input failed");

    println!("{}", ASK_FOR_VALUE);
    io::stdin()
        .read_line(&mut value)
        .expect("Reading input failed");
    let value: f32 = value.trim().parse().unwrap();

    (from, to, value)
}
