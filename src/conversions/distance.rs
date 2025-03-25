#[derive(Debug)]
pub struct InputError;

pub enum DistanceOptions {
    Km,
    Meter,
    Mile,
}

impl DistanceOptions {
    pub fn from_str(s: &str) -> Result<DistanceOptions, InputError> {
        match s {
            "1" => Ok(DistanceOptions::Km),
            "2" => Ok(DistanceOptions::Meter),
            "3" => Ok(DistanceOptions::Mile),
            _ => Err(InputError),
        }
    }
}

pub struct DistanceStruct {
    pub from: DistanceOptions,
    pub to: DistanceOptions,
    pub value: f32,
}

impl DistanceStruct {
    pub fn new(from: DistanceOptions, to: DistanceOptions, value: f32) -> DistanceStruct {
        DistanceStruct { from, to, value }
    }

    pub fn convert(&self) -> f32 {
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
