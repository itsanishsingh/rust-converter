#[derive(Debug)]
pub struct InputError;

pub enum WeightOptions {
    Kg,
    Gram,
    Pound,
}

impl WeightOptions {
    pub fn from_str(s: &str) -> Result<WeightOptions, InputError> {
        match s {
            "1" => Ok(WeightOptions::Kg),
            "2" => Ok(WeightOptions::Gram),
            "3" => Ok(WeightOptions::Pound),
            _ => Err(InputError),
        }
    }
}

pub struct WeightStruct {
    pub from: WeightOptions,
    pub to: WeightOptions,
    pub value: f32,
}

impl WeightStruct {
    pub fn new(from: WeightOptions, to: WeightOptions, value: f32) -> WeightStruct {
        WeightStruct { from, to, value }
    }

    pub fn convert(&self) -> f32 {
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
