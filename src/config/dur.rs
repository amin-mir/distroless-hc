use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_unit() {
        let error = "2000".parse::<TimeUnit>().unwrap_err();
        assert_eq!(ParseDurError::MissingUnit, error);
    }

    #[test]
    fn test_missing_numeric_part() {
        match "ms".parse::<TimeUnit>() {
            Err(ParseDurError::InvalidNum(_)) => (),
            _ => panic!("Expected InvalidNum error"),
        };
    }

    macro_rules! parse_tests {
        ($($name:ident: $dur_str:expr, $expected:expr);+) => {
            $(
                #[test]
                fn $name() {
                    let dur: TimeUnit = $dur_str.parse().unwrap();
                    assert_eq!($expected, dur.into());
                }
            )+
        };
    }

    parse_tests!(
        test_parse_ns: "1500ns", Duration::from_nanos(1500);
        test_parse_us: "500us", Duration::from_micros(500);
        test_parse_ms: "200ms", Duration::from_millis(200);
        test_parse_s: "2s", Duration::from_secs(2);
        test_parse_m: "4m", Duration::from_secs(4 * 60)
    );
}
