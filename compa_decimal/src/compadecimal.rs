use num::{PrimInt, Unsigned};
use std::{any::type_name_of_val, fmt::Display, str::FromStr};

use crate::{error::*, utils::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompaDecimal {
    value: String,
}

impl Ord for CompaDecimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let compa_digits = get_compa_digits();
        if self.value.len() != other.value.len() {
            return self.value.len().cmp(&other.value.len());
        }
        for (ac, bc) in self.value.chars().zip(other.value.chars()) {
            let ai = compa_digits.iter().position(|&x| x == ac).unwrap();
            let bi = compa_digits.iter().position(|&x| x == bc).unwrap();
            if ai != bi {
                return ai.cmp(&bi);
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for CompaDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Display for CompaDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for CompaDecimal {
    fn default() -> Self {
        Self {
            value: "0".to_string(),
        }
    }
}

impl FromStr for CompaDecimal {
    type Err = CompaDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !valid_str(s) {
            return Err(CompaDecimalError {
                error_message: "All chars have to be valid compa digits".to_string(),
            });
        }
        Ok(CompaDecimal {
            value: s.to_string(),
        })
    }
}

impl CompaDecimal {
    pub fn new() -> CompaDecimal {
        CompaDecimal {
            value: "0".to_string(),
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn decimal_to_compa<T>(mut num: T) -> Result<CompaDecimal, CompaDecimalError>
    where
        T: PrimInt + Unsigned,
    {
        let compa_digits = get_compa_digits();
        let base = T::from(compa_digits.len()).unwrap();
        let mut result = String::new();

        if num == T::zero() {
            return Ok(CompaDecimal::new());
        }

        while num > T::zero() {
            let reminder = (num % base).to_usize().unwrap();
            result.push(compa_digits[reminder]);
            num = num / base;
        }

        Ok(CompaDecimal {
            value: result.chars().rev().collect(),
        })
    }

    pub fn to_decimal<T>(&self) -> Result<T, CompaDecimalError>
    where
        T: PrimInt + Unsigned,
    {
        let value_digits: Vec<char> = self.value.chars().collect();
        let compa_digits = get_compa_digits();
        let base = T::from(compa_digits.len()).unwrap();
        let mut result: T = T::zero();

        for digit in value_digits {
            match compa_digits.iter().position(|x| x == &digit) {
                Some(position) => {
                    result = T::checked_mul(&result, &base).ok_or_else(|| CompaDecimalError {
                        error_message: format!(
                            "Overflow error! The compa value was too big to store in a {} data type",
                            type_name_of_val(&result)
                        ),
                    })?;
                    result = T::checked_add(&result, &T::from(position).unwrap()).ok_or_else(|| CompaDecimalError {
                        error_message: format!(
                            "Overflow error! The compa value was too big to store in a {} data type",
                            type_name_of_val(&result)
                        ),
                    })?;
                }
                None => {
                    return Err(CompaDecimalError {
                        error_message: format!("Invalid character: {}", digit),
                    });
                }
            }
        }

        Ok(result)
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn plus_one(&self) -> Result<CompaDecimal, CompaDecimalError> {
        let compa_digits = get_compa_digits();
        let base = compa_digits.len();
        let mut digits: Vec<char> = self.value.chars().collect();
        let mut carry = true;

        for i in (0..digits.len()).rev() {
            if carry {
                let idx = compa_digits
                    .iter()
                    .position(|&x| x == digits[i])
                    .ok_or_else(|| CompaDecimalError {
                        error_message: format!(
                            "Unexpected error! invalid char found - {}",
                            digits[i]
                        ),
                    })?;
                if idx + 1 == base {
                    digits[i] = compa_digits[0];
                    carry = true;
                } else {
                    digits[i] = compa_digits[idx + 1];
                    carry = false;
                }
            }
        }

        if carry {
            digits.insert(0, compa_digits[1]);
        }

        Ok(CompaDecimal {
            value: digits.into_iter().collect(),
        })
    }

    pub fn minus_one(&self) -> Result<CompaDecimal, CompaDecimalError> {
        let compa_digits = get_compa_digits();
        let mut digits: Vec<char> = self.value.chars().collect();

        if digits.iter().all(|&c| c == '0') {
            return Err(CompaDecimalError {
                error_message: "Cannot decrement below zero".to_string(),
            });
        }

        let mut borrow = true;
        for i in (0..digits.len()).rev() {
            if borrow {
                let idx = compa_digits
                    .iter()
                    .position(|&x| x == digits[i])
                    .ok_or_else(|| CompaDecimalError {
                        error_message: format!(
                            "Unexpected error! invalid char found - {}",
                            digits[i]
                        ),
                    })?;
                if idx == 0 {
                    digits[i] = compa_digits[compa_digits.len() - 1];
                    borrow = true;
                } else {
                    digits[i] = compa_digits[idx - 1];
                    borrow = false;
                }
            }
        }

        while digits.len() > 1 && digits[0] == '0' {
            digits.remove(0);
        }

        Ok(CompaDecimal {
            value: digits.into_iter().collect(),
        })
    }

    pub fn increase_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>
    where
        T: PrimInt + Unsigned,
    {
        let compa_amount = CompaDecimal::decimal_to_compa::<T>(amount)?;
        self.add(compa_amount.get_value())
    }

    pub fn decrease_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>
    where
        T: PrimInt + Unsigned,
    {
        let compa_amount = CompaDecimal::decimal_to_compa::<T>(amount)?;
        self.subtract(compa_amount.get_value())
    }

    pub fn add(&self, additional_value: &str) -> Result<CompaDecimal, CompaDecimalError> {
        if !valid_str(additional_value) {
            return Err(CompaDecimalError {
                error_message: "All chars have to be valid compa digits".to_string(),
            });
        }
        let compa_digits = get_compa_digits();
        let base = compa_digits.len();

        let mut a: Vec<char> = self.value.chars().collect();
        let mut b: Vec<char> = additional_value.chars().collect();

        while a.len() < b.len() {
            a.insert(0, '0');
        }
        while b.len() < a.len() {
            b.insert(0, '0');
        }

        let mut carry = 0;
        let mut result = Vec::with_capacity(a.len() + 1);

        for i in (0..a.len()).rev() {
            let ai = compa_digits.iter().position(|&x| x == a[i]).unwrap();
            let bi = compa_digits.iter().position(|&x| x == b[i]).unwrap();
            let sum = ai + bi + carry;
            result.push(compa_digits[sum % base]);
            carry = sum / base;
        }

        if carry > 0 {
            result.push(compa_digits[carry]);
        }

        result.reverse();
        Ok(CompaDecimal {
            value: result.into_iter().collect(),
        })
    }

    pub fn subtract(&self, subtrahend: &str) -> Result<CompaDecimal, CompaDecimalError> {
        if !valid_str(subtrahend) {
            return Err(CompaDecimalError {
                error_message: "All chars have to be valid compa digits".to_string(),
            });
        }
        let compa_digits = get_compa_digits();
        let base = compa_digits.len();
        match self.cmp_str(subtrahend) {
            Ok(cmp_result) => {
                if cmp_result == std::cmp::Ordering::Less {
                    return Err(CompaDecimalError {
                        error_message: "Result would be negative".to_string(),
                    });
                }
            }
            Err(error) => return Err(error),
        }

        let mut a: Vec<char> = self.value.chars().collect();
        let mut b: Vec<char> = subtrahend.chars().collect();

        while a.len() < b.len() {
            a.insert(0, '0');
        }
        while b.len() < a.len() {
            b.insert(0, '0');
        }

        let mut result = Vec::with_capacity(a.len());
        let mut borrow = 0;

        for i in (0..a.len()).rev() {
            let ai = compa_digits.iter().position(|&x| x == a[i]).unwrap() as isize;
            let bi = compa_digits.iter().position(|&x| x == b[i]).unwrap() as isize;
            let mut diff = ai - bi - borrow;
            if diff < 0 {
                diff += base as isize;
                borrow = 1;
            } else {
                borrow = 0;
            }
            result.push(compa_digits[diff as usize]);
        }

        while result.len() > 1 && result.last() == Some(&'0') {
            result.pop();
        }

        result.reverse();
        Ok(CompaDecimal {
            value: result.into_iter().collect(),
        })
    }

    pub fn cmp_str(&self, comparand: &str) -> Result<std::cmp::Ordering, CompaDecimalError> {
        if !valid_str(comparand) {
            return Err(CompaDecimalError {
                error_message: "All chars have to be valid compa digits".to_string(),
            });
        }
        let compa_digits = get_compa_digits();
        if self.value.len() != comparand.len() {
            return Ok(self.value.len().cmp(&comparand.len()));
        }
        for (ac, bc) in self.value.chars().zip(comparand.chars()) {
            let ai = compa_digits.iter().position(|&x| x == ac).unwrap();
            let bi = compa_digits.iter().position(|&x| x == bc).unwrap();
            if ai != bi {
                return Ok(ai.cmp(&bi));
            }
        }
        Ok(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn plus_one_test() {
        let compa_decimal1 = CompaDecimal::from_str("0").unwrap();
        let compa_decimal1 = compa_decimal1.plus_one().unwrap();
        assert_eq!(compa_decimal1.value, "1");
        let compa_decimal1 = compa_decimal1.plus_one().unwrap();
        assert_eq!(compa_decimal1.value, "2");
        let compa_decimal2 = CompaDecimal::from_str("9").unwrap();
        let compa_decimal2 = compa_decimal2.plus_one().unwrap();
        assert_eq!(compa_decimal2.value, "A");
        let compa_decimal3 = CompaDecimal::from_str("z").unwrap();
        let compa_decimal3 = compa_decimal3.plus_one().unwrap();
        assert_eq!(compa_decimal3.value, "!");
        let compa_decimal4 = CompaDecimal::from_str("10").unwrap();
        let compa_decimal4 = compa_decimal4.plus_one().unwrap();
        assert_eq!(compa_decimal4.value, "11");
        let compa_decimal5 = CompaDecimal::from_str("19").unwrap();
        let compa_decimal5 = compa_decimal5.plus_one().unwrap();
        assert_eq!(compa_decimal5.value, "1A");
        let compa_decimal6 = CompaDecimal::from_str("1z").unwrap();
        let compa_decimal6 = compa_decimal6.plus_one().unwrap();
        assert_eq!(compa_decimal6.value, "1!");
        let compa_decimal7 = CompaDecimal::from_str("1~").unwrap();
        let compa_decimal7 = compa_decimal7.plus_one().unwrap();
        assert_eq!(compa_decimal7.value, "20");
    }

    #[test]
    fn miuns_one_test() {
        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal1 = compa_decimal1.minus_one().unwrap();
        assert_eq!(compa_decimal1.value, "0");
        let compa_decimal2 = CompaDecimal::from_str("A").unwrap();
        let compa_decimal2 = compa_decimal2.minus_one().unwrap();
        assert_eq!(compa_decimal2.value, "9");
        let compa_decimal3 = CompaDecimal::from_str("!").unwrap();
        let compa_decimal3 = compa_decimal3.minus_one().unwrap();
        assert_eq!(compa_decimal3.value, "z");
        let compa_decimal4 = CompaDecimal::from_str("11").unwrap();
        let compa_decimal4 = compa_decimal4.minus_one().unwrap();
        assert_eq!(compa_decimal4.value, "10");
        let compa_decimal5 = CompaDecimal::from_str("1A").unwrap();
        let compa_decimal5 = compa_decimal5.minus_one().unwrap();
        assert_eq!(compa_decimal5.value, "19");
        let compa_decimal6 = CompaDecimal::from_str("1z").unwrap();
        let compa_decimal6 = compa_decimal6.minus_one().unwrap();
        assert_eq!(compa_decimal6.value, "1Z");
        let compa_decimal7 = CompaDecimal::from_str("20").unwrap();
        let compa_decimal7 = compa_decimal7.minus_one().unwrap();
        assert_eq!(compa_decimal7.value, "1~");
        let compa_decimal7 = CompaDecimal::from_str("10").unwrap();
        let compa_decimal7 = compa_decimal7.minus_one().unwrap();
        assert_eq!(compa_decimal7.value, "~");
    }

    #[test]
    fn decimal_to_compa_test() {
        let compa_decimal1 = CompaDecimal::decimal_to_compa::<u8>(16).unwrap();
        assert_eq!(compa_decimal1.value, "D");
        let compa_decimal2 = CompaDecimal::decimal_to_compa::<u32>(1329).unwrap();
        assert_eq!(compa_decimal2.value, "Cb");
        let compa_decimal3 = CompaDecimal::decimal_to_compa::<u64>(27068251).unwrap();
        assert_eq!(compa_decimal3.value, "LwOa");
        let compa_decimal4 =
            CompaDecimal::decimal_to_compa::<u128>(340282366920938463463374607431768211455)
                .unwrap();
        assert_eq!(compa_decimal4.value, "a2o~TWI*I+5G('\\99=ab");
    }

    #[test]
    fn to_decimal_test() {
        let compa_decimal1 = CompaDecimal::from_str("D").unwrap();
        assert_eq!(compa_decimal1.to_decimal::<u8>().unwrap(), 16);

        let compa_decimal2 = CompaDecimal::from_str("Cb").unwrap();
        assert_eq!(compa_decimal2.to_decimal::<u32>().unwrap(), 1329);

        let compa_decimal3 = CompaDecimal::from_str("LwOa").unwrap();
        assert_eq!(compa_decimal3.to_decimal::<u64>().unwrap(), 27068251);

        let compa_decimal4 = CompaDecimal::from_str("a2o~TWI*I+5G('\\99=ab").unwrap();
        assert_eq!(
            compa_decimal4.to_decimal::<u128>().unwrap(),
            340282366920938463463374607431768211455
        );
    }

    #[test]
    fn len_test() {
        let compa_decimal1 = CompaDecimal::from_str("123").unwrap();
        assert_eq!(compa_decimal1.len(), 3);
    }

    #[test]
    fn increase_by_test() {
        let mut compa_decimal1 = CompaDecimal::new();
        compa_decimal1 = compa_decimal1.increase_by::<u8>(1).unwrap();
        assert_eq!(compa_decimal1.value, "1");

        let mut compa_decimal2 = CompaDecimal::new();
        compa_decimal2 = compa_decimal2.increase_by::<u32>(1234).unwrap();
        assert_eq!(compa_decimal2.value, "bB");

        let mut compa_decimal3 = CompaDecimal::new();
        compa_decimal3 = compa_decimal3.increase_by::<u64>(1234567).unwrap();
        assert_eq!(compa_decimal3.value, "1r&$");

        let mut compa_decimal4 = CompaDecimal::new();
        compa_decimal4 = compa_decimal4.increase_by::<u128>(1234556778785).unwrap();
        assert_eq!(compa_decimal4.value, "1-Fq}q3");
    }

    #[test]
    fn decrease_by_test() {
        let mut compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u8>(1).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from_str("bB").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u32>(1234).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from_str("1r&$").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u64>(1234567).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from_str("1-Fq}q3").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u128>(1234556778785).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from_str("1-Fq}q3").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u8>(1).unwrap();
        assert_eq!(compa_decimal1.value, "1-Fq}q2");

        let mut compa_decimal1 = CompaDecimal::from_str("1-Fq}q3").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u16>(100).unwrap();
        assert_eq!(compa_decimal1.value, "1-Fq}p}");

        let mut compa_decimal1 = CompaDecimal::from_str("1-Fq}q3").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u32>(2395784).unwrap();
        assert_eq!(compa_decimal1.value, "1-Fp8j}");

        let mut compa_decimal1 = CompaDecimal::from_str("1-Fq}q3").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u128>(234897382497).unwrap();
        assert_eq!(compa_decimal1.value, "1Qe=6LX");
    }

    #[test]
    fn add_test() {
        let compa_decimal1 = CompaDecimal::new();
        let compa_decimal1 = compa_decimal1.add("1").unwrap();
        assert_eq!(compa_decimal1.value, "1");

        let compa_decimal1 = CompaDecimal::new();
        let compa_decimal1 = compa_decimal1.add("1AWS").unwrap();
        assert_eq!(compa_decimal1.value, "1AWS");

        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal1 = compa_decimal1.add("1").unwrap();
        assert_eq!(compa_decimal1.value, "2");

        let compa_decimal1 = CompaDecimal::from_str("aAswf").unwrap();
        let compa_decimal1 = compa_decimal1.add("AsdgrW11").unwrap();
        assert_eq!(compa_decimal1.value, "AsdMX7XG");
    }

    #[test]
    fn subtract_test() {
        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("1").unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let compa_decimal1 = CompaDecimal::from_str("1AWS").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("1AWS").unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let compa_decimal1 = CompaDecimal::from_str("2").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("1").unwrap();
        assert_eq!(compa_decimal1.value, "1");

        let compa_decimal1 = CompaDecimal::from_str("AsdMX7XG").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("AsdgrW11").unwrap();
        assert_eq!(compa_decimal1.value, "aAswf");

        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("2");
        assert!(compa_decimal1.is_err());
    }

    #[test]
    fn cmp_test() {
        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal2 = CompaDecimal::from_str("2").unwrap();
        assert!(compa_decimal1 < compa_decimal2);

        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal2 = CompaDecimal::from_str("1").unwrap();
        assert!(compa_decimal1 == compa_decimal2);

        let compa_decimal1 = CompaDecimal::from_str("1").unwrap();
        let compa_decimal2 = CompaDecimal::from_str("0").unwrap();
        assert!(compa_decimal1 > compa_decimal2);
    }

    #[test]
    fn cmp_str_test() {
        let compa_decimal1 = CompaDecimal::from_str("df$fG35SDd").unwrap();
        assert_eq!(
            compa_decimal1.cmp_str("4Dfh4hd").unwrap(),
            Ordering::Greater
        );

        let compa_decimal1 = CompaDecimal::from_str("df$fG35SDd").unwrap();
        assert_eq!(
            compa_decimal1.cmp_str("df$fG35SDd").unwrap(),
            Ordering::Equal
        );

        let compa_decimal1 = CompaDecimal::from_str("df$fG35SDd").unwrap();
        assert_eq!(
            compa_decimal1.cmp_str("df$fG35SDd$%FDgfd2d").unwrap(),
            Ordering::Less
        );
    }
}
