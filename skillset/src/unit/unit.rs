use bigdecimal::BigDecimal;

#[derive(Debug, Copy, Clone)]
pub struct UnitEntry {
    pub symbols: &'static [&'static str],
    pub scale: i32,
    pub base: BaseUnit,
    pub prefixes: &'static [UnitPrefix],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub value: BigDecimal,
    pub base: BaseUnit,
    pub prefix: Option<UnitPrefix>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BaseUnit {
    Hertz,
    Meter,
    Gram,
    Ounce,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitPrefix {
    Nano = -9,
    Micro = -6,
    Milli = -3,
    Kilo = 3,
    Mega = 6,
    Giga = 9,
    Tera = 12,
}
