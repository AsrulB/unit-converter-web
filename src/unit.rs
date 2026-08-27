#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Length {
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Weight {
    Kilogram,
    Gram,
    Milligram,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Temperature {
    Celcius,
    Fahrenheit,
    Kelvin,
}
