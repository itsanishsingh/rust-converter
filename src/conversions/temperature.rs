#[derive(Debug)]
pub struct InputError;

pub enum TemperatureOptions {
    Celcius,
    Kelvin,
    Fahrenheit,
}

impl TemperatureOptions {
    pub fn from_str(s: &str) -> Result<TemperatureOptions, InputError> {
        match s {
            "1" => Ok(TemperatureOptions::Celcius),
            "2" => Ok(TemperatureOptions::Kelvin),
            "3" => Ok(TemperatureOptions::Fahrenheit),
            _ => Err(InputError),
        }
    }
}

pub struct TemperatureStruct {
    pub from: TemperatureOptions,
    pub to: TemperatureOptions,
    pub value: f32,
}

impl TemperatureStruct {
    pub fn new(from: TemperatureOptions, to: TemperatureOptions, value: f32) -> TemperatureStruct {
        TemperatureStruct { from, to, value }
    }

    pub fn convert(&self) -> f32 {
        match self.from {
            TemperatureOptions::Celcius => match self.to {
                TemperatureOptions::Celcius => self.value,
                TemperatureOptions::Kelvin => self.value + 273.15,
                TemperatureOptions::Fahrenheit => (self.value * 9 as f32) / 5.0 + 32 as f32,
            },
            TemperatureOptions::Kelvin => match self.to {
                TemperatureOptions::Celcius => self.value - 273.15,
                TemperatureOptions::Kelvin => self.value,
                TemperatureOptions::Fahrenheit => {
                    ((self.value - 273.15) * 9 as f32) / 5.0 + 32 as f32
                }
            },
            TemperatureOptions::Fahrenheit => match self.to {
                TemperatureOptions::Celcius => (self.value - 32 as f32) * (5 / 9) as f32,
                TemperatureOptions::Kelvin => (self.value - 32 as f32) * (5 / 9) as f32 + 273.15,
                TemperatureOptions::Fahrenheit => self.value,
            },
        }
    }
}
