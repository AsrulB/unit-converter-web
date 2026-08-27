use crate::unit::{
    Length::{self, *},
    Temperature::*,
    Weight,
    Weight::{Gram, Kilogram, Milligram},
};
use crate::weight_data::WeightData;

pub mod unit;
pub mod weight_data;

fn main() {
    let data = WeightData {
        num: 2.0,
        from: Gram,
        to: Milligram,
    };
    println!("{}", data.convert());
}
