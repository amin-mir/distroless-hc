use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Duration;

fn parse_duration(val: &str) -> Result<Duration, ParseDurError> {
    let idx = val
        .find(|c: char| !c.is_digit(10))
        .ok_or(ParseDurError::MissingUnit)?;

    let (num, unit) = val.split_at(idx);

    let num: u64 = num.parse()?;
    let unit: Unit = unit.parse()?;

    Ok(to_duration(num, unit))
}

fn to_duration(num: u64, unit: Unit) -> Duration {
    match unit {
        Unit::Nanos => Duration::from_nanos(num),
        Unit::Micros => Duration::from_micros(num),
        Unit::Millis => Duration::from_millis(num),
        Unit::Secs => Duration::from_secs(num),
        Unit::Mins => Duration::from_secs(num * 60),
    }
}

enum Unit {
    Nanos,
    Micros,
    Millis,
    Secs,
    Mins,
}

impl FromStr for Unit {
    type Err = ParseDurError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ns" => Ok(Unit::Nanos),
            "us" => Ok(Unit::Micros),
            "ms" => Ok(Unit::Millis),
            "s" => Ok(Unit::Secs),
            "m" => Ok(Unit::Mins),
            _ => Err(ParseDurError::InvalidUnit),
        }
    }
}

#[derive(Debug)]
enum ParseDurError {
    MissingUnit,
    InvalidNum(ParseIntError),
    InvalidUnit,
}

impl std::error::Error for ParseDurError {}

impl std::fmt::Display for ParseDurError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseDurError::MissingUnit => write!(f, "Duration unit is missing"),
            ParseDurError::InvalidNum(e) => write!(f, "{}", e),
            ParseDurError::InvalidUnit => write!(f, "Duration unit is in valid"),
        }
    }
}

impl From<ParseIntError> for ParseDurError {
    fn from(err: ParseIntError) -> Self {
        ParseDurError::InvalidNum(err)
    }
}
