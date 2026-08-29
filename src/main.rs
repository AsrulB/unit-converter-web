use crate::unit::Weight::{Ounce, Pound};
use crate::unit_data::{TemperatureData, WeightData};
use crate::{
    unit::{
        Length::{self, *},
        Temperature::*,
        Weight::{self, Gram, Kilogram, Milligram},
    },
    unit_data::LengthData,
};

pub mod unit;
pub mod unit_data;

fn main() {
    let data = WeightData {
        num: 1.0,
        from: Kilogram,
        to: Ounce,
    };
    println!("{}", data.convert());
}
