#[derive(Hash, PartialEq, Eq, Debug)]
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

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Weight {
    Pound,
    Ounce,
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
