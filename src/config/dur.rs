use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Duration;

pub enum TimeUnit {
    Nanos(u64),
    Micros(u64),
    Millis(u64),
    Secs(u64),
    Mins(u64),
}

impl From<TimeUnit> for Duration {
    fn from(unit: TimeUnit) -> Duration {
        match unit {
            TimeUnit::Nanos(num) => Duration::from_nanos(num),
            TimeUnit::Micros(num) => Duration::from_micros(num),
            TimeUnit::Millis(num) => Duration::from_millis(num),
            TimeUnit::Secs(num) => Duration::from_secs(num),
            TimeUnit::Mins(num) => Duration::from_secs(num * 60),
        }
    }
}

impl FromStr for TimeUnit {
    type Err = ParseDurError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let idx = val
            .find(|c: char| !c.is_digit(10))
            .ok_or(ParseDurError::MissingUnit)?;

        let (num, unit) = val.split_at(idx);

        let num: u64 = num.parse()?;

        match unit {
            "ns" => Ok(TimeUnit::Nanos(num)),
            "us" => Ok(TimeUnit::Micros(num)),
            "ms" => Ok(TimeUnit::Millis(num)),
            "s" => Ok(TimeUnit::Secs(num)),
            "m" => Ok(TimeUnit::Mins(num)),
            _ => Err(ParseDurError::InvalidUnit),
        }
    }
}

#[derive(Debug)]
pub enum ParseDurError {
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
