use std::fmt;

use crate::common::prelude::Number;

#[derive(Debug)]
pub enum ConversionError {
    InvalidBase,
    NumberTooLarge,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBase => write!(f, "Base must be greater than 1"),
            Self::NumberTooLarge => write!(f, "Number too large for conversion"),
        }
    }
}

impl std::error::Error for ConversionError {}

pub trait BaseConversion: Number {
    fn to_base(&self, base: Self) -> Result<String, ConversionError> {
        if base <= Self::one() {
            return Err(ConversionError::InvalidBase);
        }

        if *self == Self::zero() {
            return Ok("0".to_string());
        }

        let mut n = *self;
        let mut is_negative = false;

        if n < Self::zero() {
            is_negative = true;
            n = n.abs();
        }

        let mut digits = Vec::new();
        while n > Self::zero() {
            let remainder = n % base;
            let digit = if remainder < Self::from(10).unwrap() {
                (remainder.to_u8().unwrap() + b'0') as char
            } else {
                (remainder.to_u8().unwrap() - 10 + b'a') as char
            };
            digits.push(digit);
            n = n / base;
        }

        let mut result = if is_negative {
            String::from("-")
        } else {
            String::new()
        };

        result.extend(digits.into_iter().rev());
        Ok(result)
    }

    fn to_base_fixed(&self, base: Self, min_length: usize) -> Result<String, ConversionError> {
        let mut result = self.to_base(base)?;
        
        if result.starts_with('-') {
            let num_part = &result[1..];
            if num_part.len() < min_length {
                result = format!("-{}{}", "0".repeat(min_length - num_part.len()), num_part);
            }
        } else if result.len() < min_length {
            result = format!("{}{}", "0".repeat(min_length - result.len()), result);
        }
        
        Ok(result)
    }

    fn to_binary_fixed(&self, length: usize) -> Result<String, ConversionError> {
        self.to_base_fixed(Self::from(2).unwrap(), length)
    }

    fn to_ternary_fixed(&self, length: usize) -> Result<String, ConversionError> {
        self.to_base_fixed(Self::from(3).unwrap(), length)
    }

    fn to_binary(&self) -> Result<String, ConversionError> {
        self.to_base(Self::from(2).unwrap())
    }

    fn to_ternary(&self) -> Result<String, ConversionError> {
        self.to_base(Self::from(3).unwrap())
    }

    fn as_binary(&self) -> Result<String, ConversionError> {
        Ok(format!("0b{}", self.to_binary()?))
    }

    fn as_ternary(&self) -> Result<String, ConversionError> {
        Ok(format!("0t{}", self.to_ternary()?))
    }
}

impl<T: Number> BaseConversion for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_conversion() {
        assert_eq!(42i32.to_binary().unwrap(), "101010");
        assert_eq!((-42i32).to_binary().unwrap(), "-101010");
        assert_eq!(0i32.to_binary().unwrap(), "0");
    }

    #[test]
    fn test_ternary_conversion() {
        assert_eq!(42i32.to_ternary().unwrap(), "1120");
        assert_eq!((-42i32).to_ternary().unwrap(), "-1120");
        assert_eq!(0i32.to_ternary().unwrap(), "0");
    }

    #[test]
    fn test_prefixed_representations() {
        assert_eq!(42i32.as_binary().unwrap(), "0b101010");
        assert_eq!(42i32.as_ternary().unwrap(), "0t1120");
    }

    #[test]
    fn test_invalid_base() {
        assert!(42i32.to_base(0i32).is_err());
        assert!(42i32.to_base(1i32).is_err());
    }

    #[test]
    fn test_fixed_length() {
        assert_eq!(5i32.to_binary_fixed(4).unwrap(), "0101");
        assert_eq!((-5i32).to_binary_fixed(4).unwrap(), "-0101");
        assert_eq!(0i32.to_binary_fixed(3).unwrap(), "000");
        assert_eq!(2i32.to_ternary_fixed(3).unwrap(), "002");
    }
}