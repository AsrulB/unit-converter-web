#[derive(Clone, Hash, PartialEq, Eq, Debug)]
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

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Weight {
    Pound,
    Ounce,
    Kilogram,
    Gram,
    Milligram,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Temperature {
    Celcius,
    Fahrenheit,
    Kelvin,
}
