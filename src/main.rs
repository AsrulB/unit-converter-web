use crate::unit::Weight::{Ounce, Pound};
use crate::unit::{
    Length::{self, *},
    Temperature::*,
    Weight::{self, Gram, Kilogram, Milligram},
};
use crate::unit_data::{ConversionData, TemperatureData};

pub mod unit;
pub mod unit_data;

fn main() {
    let data = TemperatureData {
        value: 30.0,
        from: Celcius,
        to: Fahrenheit,
    };
    println!("{}", data.convert());
}
