use std::num::ParseFloatError;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sets {
    pub weight_kg: f32,
    pub reps: i64,
}

#[derive(thiserror::Error, derive_more::Display, Debug, PartialEq)]
pub enum SetsParseError {
    ParseError(#[from] regex::Error),
    CapError,
    ParseIntError(#[from] std::num::ParseIntError),
    ParseFloatError(#[from] ParseFloatError),
}
impl FromStr for Sets {
    type Err = SetsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(\s*)(?<weight>[0-9]*[.,]?[0-9]+)(\s+)(?<reps>[0-9]+)(\s*)$",
        )?;
        let cap = re.captures(s).ok_or(SetsParseError::CapError)?;

        Ok(Self {
            weight_kg: cap["weight"].parse()?,
            reps: cap["reps"].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let sets: Sets = "20 20".parse().unwrap();
        assert_eq!(sets.reps, 20);
        assert_eq!(sets.weight_kg, 20f32);

        let sets: Sets = "20      20".parse().unwrap();
        assert_eq!(sets.reps, 20);
        assert_eq!(sets.weight_kg, 20f32);

        let sets: Sets = "  20      20".parse().unwrap();
        assert_eq!(sets.reps, 20);
        assert_eq!(sets.weight_kg, 20f32);

        let sets: Sets = "20      20   ".parse().unwrap();
        assert_eq!(sets.reps, 20);
        assert_eq!(sets.weight_kg, 20f32);
    }

    #[test]
    fn parse_test_error() {
        let sets = "20      s20".parse::<Sets>();
        assert!(sets.is_err());

        let sets = "20ss      20".parse::<Sets>();
        assert!(sets.is_err());
    }
    #[test]
    fn test_from_str_valid_input() {
        let input = "  10.5  15  ";
        let expected = Sets {
            weight_kg: 10.5,
            reps: 15,
        };
        assert_eq!(Sets::from_str(input), Ok(expected));
    }

    #[test]
    fn test_from_str_invalid_input() {
        let input = "invalid input";
        assert!(Sets::from_str(input).is_err());
    }

    #[test]
    fn test_from_str_empty_input() {
        let input = "";
        assert!(Sets::from_str(input).is_err());
    }

    #[test]
    fn test_from_str_no_weight() {
        let input = "  15  ";
        assert!(Sets::from_str(input).is_err());
    }

    #[test]
    fn test_from_str_no_reps() {
        let input = "  10.5  ";
        assert!(Sets::from_str(input).is_err());
    }

    #[test]
    fn test_from_str_zero_weight() {
        let input = "  0  15  ";
        let expected = Sets {
            weight_kg: 0.0,
            reps: 15,
        };
        assert_eq!(Sets::from_str(input), Ok(expected));
    }

    #[test]
    fn test_from_str_zero_reps() {
        let input = "  10.5  0  ";
        let expected = Sets {
            weight_kg: 10.5,
            reps: 0,
        };
        assert_eq!(Sets::from_str(input), Ok(expected));
    }
}
