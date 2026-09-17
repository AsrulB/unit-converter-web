use crate::unit::Length;
use crate::unit::Temperature;
// use crate::unit::Temperature::{Celcius, Fahrenheit, Kelvin};
use crate::unit::Weight;
use math::round;
use ordered_float::OrderedFloat;
use serde_json::Value;
use serde_json::json;
use std::hash::Hash;

pub enum DataType {
    Length(LengthData),
    Weight(WeightData),
    Temperature(TemperatureData),
}

impl DataType {
    fn convert(&self) -> Option<Value> {
        match self {
            Self::Length(data) => Some(data.convert()),
            Self::Weight(data) => Some(data.convert()),
            Self::Temperature(data) => Some(data.convert()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WeightData {
    pub from: Weight,
    pub to: Weight,
    pub value: OrderedFloat<f64>,
}

impl WeightData {
    fn convert(&self) -> Value {
        let base_value = self.value * self.from.rate();
        let result = base_value / self.to.rate();
        let new_value = round::half_away_from_zero(*result, 3);

        json!({
            "success": true,
            "message": "Length conversion success!",
            "data": {
                "value": new_value,
                "from": self.from,
                "to": self.to
            }
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LengthData {
    pub from: Length,
    pub to: Length,
    pub value: OrderedFloat<f64>,
}

impl LengthData {
    fn convert(&self) -> Value {
        let base_value = self.value * self.from.rate();
        let result = base_value / self.to.rate();
        let new_value = round::half_away_from_zero(*result, 3);

        json!({
            "success": true,
            "message": "Length conversion success!",
            "data": {
                "value": new_value,
                "from": self.from,
                "to": self.to
            }
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TemperatureData {
    pub from: Temperature,
    pub to: Temperature,
    pub value: OrderedFloat<f64>,
}

impl TemperatureData {
    fn convert(&self) -> Value {
        let kelvin = Temperature::to_kelvin(self);
        let result = Temperature::from_kelvin(kelvin, self.to);

        json!({
            "success": true,
            "message": "Temperature conversion success!",
            "data": {
                "value": result,
                "from": self.from,
                "to": self.to
            }
        })
    }
}

pub struct ConversionData {
    pub data: DataType,
}

impl ConversionData {
    pub fn run(val: Value) -> Option<Value> {
        match Self::extract(val) {
            None => None,
            Some(data) => data.convert(),
        }
    }

    fn extract(val: Value) -> Option<DataType> {
        let from = val
            .get("from_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let to = val
            .get("to_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let value = val
            .get("value")
            .and_then(|v| v.as_str())
            .map(|s| s.parse::<f64>().unwrap_or_default())
            .unwrap();

        match Self::build_unit_data(from, to, value) {
            None => None,
            Some(data) => Some(data),
        }
    }

    fn build_unit_data(from: &str, to: &str, value: f64) -> Option<DataType> {
        if let Some(f) = Length::from_str(from) {
            if let Some(t) = Length::from_str(to) {
                let new_data = LengthData {
                    from: f,
                    to: t,
                    value: OrderedFloat(value),
                };
                return Some(DataType::Length(new_data));
            }
        }

        if let Some(f) = Weight::from_str(from) {
            if let Some(t) = Weight::from_str(to) {
                let new_data = WeightData {
                    from: f,
                    to: t,
                    value: OrderedFloat(value),
                };
                return Some(DataType::Weight(new_data));
            }
        }

        if let Some(f) = Temperature::from_str(from) {
            if let Some(t) = Temperature::from_str(to) {
                let new_data = TemperatureData {
                    from: f,
                    to: t,
                    value: OrderedFloat(value),
                };
                return Some(DataType::Temperature(new_data));
            }
        }

        None
    }
}
