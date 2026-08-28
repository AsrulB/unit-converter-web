use crate::unit::{
    Length::{self, *},
    Temperature::*,
    Weight::{self, Gram, Kilogram, Milligram},
};
use crate::unit_data::TemperatureData;

pub mod unit;
pub mod unit_data;
fn main() {
    let data = TemperatureData {
        num: 30.0,
        from: Celcius,
        to: Fahrenheit,
    };
    println!("{}", data.convert());
}
