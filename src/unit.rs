use crate::unit_data::TemperatureData;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
pub enum Length {
    Mile,
    Kilometer,
    Meter,
    Yard,
    Feet,
    Inch,
    Centimeter,
    Millimeter,
}

impl Length {
    pub fn rate(&self) -> f64 {
        match self {
            Length::Mile => 1.61,
            Length::Kilometer => 1.0,
            Length::Meter => 0.001,
            Length::Yard => 0.0009144,
            Length::Feet => 0.0003048,
            Length::Inch => 0.0000254,
            Length::Centimeter => 0.00001,
            Length::Millimeter => 0.000001,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mile" => Some(Self::Mile),
            "kilometer" => Some(Self::Kilometer),
            "meter" => Some(Self::Meter),
            "yard" => Some(Self::Yard),
            "feet" => Some(Self::Feet),
            "inch" => Some(Self::Inch),
            "centimeter" => Some(Self::Centimeter),
            "millimeter" => Some(Self::Millimeter),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
pub enum Weight {
    Pound,
    Ounce,
    Kilogram,
    Gram,
    Milligram,
}

impl Weight {
    pub fn rate(&self) -> f64 {
        match self {
            Weight::Kilogram => 1.0,
            Weight::Gram => 0.001,
            Weight::Pound => 0.4536,
            Weight::Ounce => 0.02835,
            Weight::Milligram => 0.000001,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kilogram" => Some(Self::Kilogram),
            "gram" => Some(Self::Gram),
            "pound" => Some(Self::Pound),
            "ounce" => Some(Self::Ounce),
            "milligram" => Some(Self::Milligram),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize)]
pub enum Temperature {
    Celcius,
    Fahrenheit,
    Kelvin,
}

impl Temperature {
    pub fn to_kelvin(data: &TemperatureData) -> f64 {
        match data.from {
            Temperature::Kelvin => *data.value,
            Temperature::Celcius => *data.value + 273.15,
            Temperature::Fahrenheit => (*data.value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }

    pub fn from_kelvin(kelvin: f64, unit: Temperature) -> f64 {
        match unit {
            Temperature::Kelvin => kelvin,
            Temperature::Celcius => kelvin - 273.15,
            Temperature::Fahrenheit => (kelvin - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kelvin" => Some(Self::Kelvin),
            "celcius" => Some(Self::Celcius),
            "fahrenheit" => Some(Self::Fahrenheit),
            _ => None,
        }
    }
}
