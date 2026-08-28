use crate::unit::{
    Temperature::{Celcius, Fahrenheit, Kelvin},
    *,
};
use std::collections::HashMap;

pub struct WeightData {
    pub num: f64,
    pub from: Weight,
    pub to: Weight,
}

pub struct LengthData {
    pub num: f64,
    pub from: Length,
    pub to: Length,
}

pub struct TemperatureData {
    pub num: f64,
    pub from: Temperature,
    pub to: Temperature,
}

impl WeightData {
    pub fn convert(&self) -> f64 {
        let mut map_length = HashMap::new();
        map_length.insert(Weight::Kilogram, 1.0);
        map_length.insert(Weight::Gram, 0.001);
        map_length.insert(Weight::Milligram, 0.000001);

        let in_kg = self.num * map_length[&self.from];
        let conversion = in_kg / map_length[&self.to];
        dbg!(&in_kg);
        conversion.round()
    }
}

impl LengthData {
    pub fn convert(&self) -> f64 {
        let mut map_length = HashMap::new();
        map_length.insert(Length::Kilometer, 1.0);
        map_length.insert(Length::Meter, 0.001);
        map_length.insert(Length::Centimeter, 0.00001);
        map_length.insert(Length::Millimeter, 0.000001);

        let in_km = self.num * map_length[&self.from];
        let conversion = in_km / map_length[&self.to];
        dbg!(&in_km);
        conversion.round()
    }
}

// To be implemented
impl TemperatureData {
    pub fn convert(&self) -> f64 {
        let conversion = (&self.from, &self.to);

        match conversion {
            (Celcius, Fahrenheit) => Self::celcius_to_fahrenheit(self.num),
            (Fahrenheit, Celcius) => Self::fahrenheit_to_celcius(self.num),
            (Celcius, Kelvin) => Self::celcius_to_kelvin(self.num),
            (Kelvin, Celcius) => Self::kelvin_to_celcius(self.num),
            (Kelvin, Fahrenheit) => Self::kelvin_to_fahrenheit(self.num),
            (Fahrenheit, Kelvin) => Self::fahrenheit_to_kelvin(self.num),
            _ => self.num,
        }
    }

    fn celcius_to_fahrenheit(t: f64) -> f64 {
        t * (9.0 / 5.0) + 32.0
    }

    fn fahrenheit_to_celcius(t: f64) -> f64 {
        (t - 32.0) * 5.0 / 9.0
    }

    fn celcius_to_kelvin(t: f64) -> f64 {
        t + 273.15
    }

    fn kelvin_to_celcius(t: f64) -> f64 {
        t - 273.15
    }

    fn kelvin_to_fahrenheit(t: f64) -> f64 {
        (t - 273.15) * 9.0 / 5.0 + 32.0
    }

    fn fahrenheit_to_kelvin(t: f64) -> f64 {
        (t - 32.0) * 5.0 / 9.0 + 273.15
    }
}
