use crate::unit::{
    Temperature::{Celcius, Fahrenheit, Kelvin},
    *,
};
use math::round;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Convertible: Clone + Eq + Hash {
    fn rate(&self) -> f64;
}

impl Convertible for Weight {
    fn rate(&self) -> f64 {
        match self {
            Weight::Kilogram => 1.0,
            Weight::Gram => 0.001,
            Weight::Pound => 0.4536,
            Weight::Ounce => 0.02835,
            Weight::Milligram => 0.000001,
        }
    }
}

impl Convertible for Length {
    fn rate(&self) -> f64 {
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
}

pub struct ConversionData<T: Convertible> {
    pub from: T,
    pub to: T,
    pub value: f64,
}

impl<T: Convertible> ConversionData<T> {
    pub fn convert(&self) -> f64 {
        let base_value = self.value * self.from.rate();
        let result = base_value / self.to.rate();
        round::half_away_from_zero(result, 3)
    }
}

pub struct TemperatureData {
    pub value: f64,
    pub from: Temperature,
    pub to: Temperature,
}

impl TemperatureData {
    pub fn convert(&self) -> f64 {
        if self.from == self.to {
            return self.value;
        }

        // convert to Kelvin as the base unit
        let kelvin = self.to_kelvin(self.value, self.from.clone());
        // convert from kelvin to target unit
        self.from_kelvin(kelvin, self.to.clone())
    }

    fn to_kelvin(&self, value: f64, unit: Temperature) -> f64 {
        match unit {
            Temperature::Kelvin => value,
            Temperature::Celcius => value + 273.15,
            Temperature::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }

    fn from_kelvin(&self, kelvin: f64, unit: Temperature) -> f64 {
        match unit {
            Temperature::Kelvin => kelvin,
            Temperature::Celcius => kelvin - 273.15,
            Temperature::Fahrenheit => (kelvin - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }
}
