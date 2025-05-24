use std::{any::{type_name, type_name_of_val}, ops::Sub};

use num::{PrimInt, Unsigned};

#[derive(Debug)]
pub struct CompaDecimalError {
    error_message: String
}
pub struct CompaDecimal {
    pub value: String 
}

impl CompaDecimal {
    fn new() -> CompaDecimal {
        CompaDecimal { 
            value: "0".to_string() 
        }
    }

    pub fn from(value: &str) -> Result<CompaDecimal, CompaDecimalError> {
        Ok(CompaDecimal { 
            value: value.to_string() 
        })
    }

    pub fn plus_one(&mut self) {
        let mut digits: Vec<char> = self.value.chars().collect();
        let digits_len: usize = digits.len();


        if self.value.len() == 1 {
            let updated_value = get_next(&digits[0]);
            match updated_value {
                '0' => self.value = "10".to_string(),
                _ => self.value = updated_value.to_string()
            }
            return;
        }

        for i in 1..(digits_len + 1) {
            let digits_len = &digits.len() - i;
            let updated_value = get_next(&digits[digits_len]);
    
            match updated_value {
                '0' => digits[digits_len] = '0',
                _ => {
                    digits[digits_len] = updated_value;
                    self.value = digits.into_iter().collect::<String>();
                    return;
                }
            }
        }
        self.value = digits.into_iter().collect::<String>();
            
    }

    pub fn minus_one(&self) -> Result<CompaDecimal, CompaDecimalError> {
        let mut digits: Vec<char> = self.value.chars().collect();
        let digits_len: usize = digits.len();


        if self.value.len() == 1 {
            let updated_value = get_previous(&digits[0]);
            match updated_value {
                Some(value) => return Ok(
                    CompaDecimal { 
                        value: value.to_string() 
                    }),
                None => return Err(
                    CompaDecimalError 
                    { error_message: "Error! Value can not be less than 0".to_string() 
                })
            }
        }

        let updated_value = get_previous(&digits[digits_len - 1]);

        match updated_value {
            Some(value) => {
                digits[digits_len - 1] = value;
                return Ok(CompaDecimal {
                    value: digits.into_iter().collect::<String>()});
            },
            None => {
                _ = {
                    if digits[digits_len - 2] == '1' {
                        digits.remove(digits_len - 1);
                        digits[digits_len - 2] = '~';
                        return Ok(CompaDecimal { value: digits.into_iter().collect::<String>() })
                    }
                    digits[digits_len - 1] = '~';
                    digits[digits_len - 2] = match get_previous(&digits[digits_len - 2]) {
                        Some(value) => value,
                        None => return Err(CompaDecimalError { error_message: "Fatal error! second value before end of sequence was 0 or invalid".to_string() })
                    };
                }
            }
        }
    

        Ok(CompaDecimal {
            value: digits.into_iter().collect::<String>()
        })
    }

    pub fn decimal_to_compa<T>(mut num: T) -> Result<CompaDecimal, CompaDecimalError>
    where T: PrimInt + Unsigned {
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

        Ok(CompaDecimal { value: result.chars().rev().collect() })
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

    pub fn increase_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>
    where
        T: PrimInt + Unsigned,
    {
        let current_value = self.to_decimal::<u128>()?;
        let amount_as_u128 = T::to_u128(&amount).ok_or_else(|| CompaDecimalError {
            error_message: "Failed to convert the amount to u128".to_string(),
        })?;
        let new_value = current_value
            .checked_add(amount_as_u128)
            .ok_or_else(|| CompaDecimalError {
                error_message: "Overflow occurred while adding the amount".to_string(),
            })?;

        CompaDecimal::decimal_to_compa(new_value)
    }

    pub fn decrease_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>
    where 
        T: PrimInt + Unsigned,
    {
        let current_value = self.to_decimal::<u128>()?;
        let amount_as_u128 = T::to_u128(&amount).ok_or_else(|| CompaDecimalError {
            error_message: "Failed to convert the amount to u128".to_string(),
        })?;

        let new_value = current_value.sub(amount_as_u128);
        CompaDecimal::decimal_to_compa::<u128>(new_value)
    }

    pub fn add(&self, additional_value: &str) -> CompaDecimal {
        let compa_digits = get_compa_digits();
        let base = compa_digits.len();

        let mut a: Vec<char> = self.value.chars().collect();
        let mut b: Vec<char> = additional_value.chars().collect();

        while a.len() < b.len() { a.insert(0, '0'); }
        while b.len() < a.len() { b.insert(0, '0'); }

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
        CompaDecimal { value: result.into_iter().collect() }
    }
}

fn get_compa_digits() -> Vec<char> {
    "0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!\"#$%&'()*+,-./:;<=>?@[\\]^_`|}{~".chars().collect()
}

fn get_next(current: &char) -> char {
    let digits: Vec<char> = get_compa_digits();
    let index = match digits.iter().position(|digit| digit == current) {
        Some(index) => index,
        None => 0
    };
    
    if index == (digits.len() - 1) {
        return '0';
    }

    digits[index + 1]
}

fn get_previous(current: &char) -> Option<char> {
    let digits: Vec<char> = get_compa_digits();
    let index = match digits.iter().position(|digit| digit == current) {
        Some(index) => index,
        None => return None
    };

    if index == 0 {
        return None
    }

    Some(digits[index - 1])
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn get_next_test() {
        assert_eq!(get_next(&'0'), '1');
        assert_eq!(get_next(&'9'), 'A');
        assert_eq!(get_next(&'A'), 'a');
        assert_eq!(get_next(&'~'), '0');
    }

    #[test]
    fn get_previous_test() {
        assert_eq!(get_previous(&'1'), Some('0'));
        assert_eq!(get_previous(&'A'), Some('9'));
        assert_eq!(get_previous(&'a'), Some('A'));
        assert_eq!(get_previous(&'0'), None);
    }

    #[test]
    fn plus_one_test() {
        let mut compa_decimal1 = CompaDecimal::from("0").unwrap();
        compa_decimal1.plus_one();
        assert_eq!(compa_decimal1.value, "1");
        compa_decimal1.plus_one();
        assert_eq!(compa_decimal1.value, "2");
        let mut compa_decimal2 = CompaDecimal::from("9").unwrap();
        compa_decimal2.plus_one();
        assert_eq!(compa_decimal2.value, "A");
        let mut compa_decimal3 = CompaDecimal::from("z").unwrap();
        compa_decimal3.plus_one();
        assert_eq!(compa_decimal3.value, "!");
        let mut compa_decimal4 = CompaDecimal::from("10").unwrap();
        compa_decimal4.plus_one();
        assert_eq!(compa_decimal4.value, "11");
        let mut compa_decimal5 = CompaDecimal::from("19").unwrap();
        compa_decimal5.plus_one();
        assert_eq!(compa_decimal5.value, "1A");
        let mut compa_decimal6 = CompaDecimal::from("1z").unwrap();
        compa_decimal6.plus_one();
        assert_eq!(compa_decimal6.value, "1!");
        let mut compa_decimal7 = CompaDecimal::from("1~").unwrap();
        compa_decimal7.plus_one();
        assert_eq!(compa_decimal7.value, "20");
    }

    #[test]
    fn miuns_one_test() {
        let mut compa_decimal1 = CompaDecimal::from("1").unwrap();
        let compa_decimal1 = compa_decimal1.minus_one().unwrap();
        assert_eq!(compa_decimal1.value, "0");
        let mut compa_decimal2 = CompaDecimal::from("A").unwrap();
        let compa_decimal2 = compa_decimal2.minus_one().unwrap();
        assert_eq!(compa_decimal2.value, "9");
        let mut compa_decimal3 = CompaDecimal::from("!").unwrap();
        let compa_decimal3 = compa_decimal3.minus_one().unwrap();
        assert_eq!(compa_decimal3.value, "z");
        let mut compa_decimal4 = CompaDecimal::from("11").unwrap();
        let compa_decimal4 = compa_decimal4.minus_one().unwrap();
        assert_eq!(compa_decimal4.value, "10");
        let mut compa_decimal5 = CompaDecimal::from("1A").unwrap();
        let compa_decimal5 = compa_decimal5.minus_one().unwrap();
        assert_eq!(compa_decimal5.value, "19");
        let mut compa_decimal6 = CompaDecimal::from("1z").unwrap();
        let compa_decimal6 = compa_decimal6.minus_one().unwrap();
        assert_eq!(compa_decimal6.value, "1Z");
        let mut compa_decimal7 = CompaDecimal::from("20").unwrap();
        let compa_decimal7 = compa_decimal7.minus_one().unwrap();
        assert_eq!(compa_decimal7.value, "1~");
        let mut compa_decimal7 = CompaDecimal::from("10").unwrap();
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
        let compa_decimal4 = CompaDecimal::decimal_to_compa::<u128>(340282366920938463463374607431768211455).unwrap();
        assert_eq!(compa_decimal4.value, "a2o~TWI*I+5G('\\99=ab");

    }

    #[test]
    fn to_decimal_test() {
        let compa_decimal1 = CompaDecimal::from("D").unwrap();
        assert_eq!(compa_decimal1.to_decimal::<u8>().unwrap(), 16);

        let compa_decimal2 = CompaDecimal::from("Cb").unwrap();
        assert_eq!(compa_decimal2.to_decimal::<u32>().unwrap(), 1329);
        
        let compa_decimal3 = CompaDecimal::from("LwOa").unwrap();
        assert_eq!(compa_decimal3.to_decimal::<u64>().unwrap(), 27068251);
        
        let compa_decimal4 = CompaDecimal::from("a2o~TWI*I+5G('\\99=ab").unwrap();
        assert_eq!(compa_decimal4.to_decimal::<u128>().unwrap(), 340282366920938463463374607431768211455);
    }

    #[test]
    fn len_test() {
        let compa_decimal1 = CompaDecimal::from("123").unwrap();
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
        let mut compa_decimal1 = CompaDecimal::from("1").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u8>(1).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from("bB").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u32>(1234).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from("1r&$").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u64>(1234567).unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let mut compa_decimal1 = CompaDecimal::from("1-Fq}q3").unwrap();
        compa_decimal1 = compa_decimal1.decrease_by::<u128>(1234556778785).unwrap();
        assert_eq!(compa_decimal1.value, "0");
    }

    #[test]
    fn Add_test() {
        let compa_decimal1 = CompaDecimal::new();
        let compa_decimal1 = compa_decimal1.add("1");
        assert_eq!(compa_decimal1.value, "1");

        let compa_decimal1 = CompaDecimal::new();
        let compa_decimal1 = compa_decimal1.add("1AWS");
        assert_eq!(compa_decimal1.value, "1AWS");

        let compa_decimal1 = CompaDecimal::from("1").unwrap();
        let compa_decimal1 = compa_decimal1.add("1");
        assert_eq!(compa_decimal1.value, "2");
    
    
        let compa_decimal1 = CompaDecimal::from("aAswf").unwrap();
        let compa_decimal1 = compa_decimal1.add("AsdgrW11");
        assert_eq!(compa_decimal1.value, "AsdMX7XG");
    }

    fn subtract_test() {
        let compa_decimal1 = CompaDecimal::from("1").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("1").unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let compa_decimal1 = CompaDecimal::from("1AWS").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("1AWS").unwrap();
        assert_eq!(compa_decimal1.value, "0");

        let compa_decimal1 = CompaDecimal::from("2").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("1").unwrap();
        assert_eq!(compa_decimal1.value, "1");
    
    
        let compa_decimal1 = CompaDecimal::from("AsdMX7XG").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("AsdgrW11").unwrap();
        assert_eq!(compa_decimal1.value, "aAswf");

        let compa_decimal1 = CompaDecimal::from("1").unwrap();
        let compa_decimal1 = compa_decimal1.subtract("2");
        assert!(compa_decimal1, CompaDecimalError);
    }

    fn cmp_test() {
        let compa_decimal1 = CompaDecimal::from("1").unwrap();
        assert_eq!(compa_decimal1.cmp("2"), Ordering::Less);

        let compa_decimal1 = CompaDecimal::from("1").unwrap();
        assert_eq!(compa_decimal1.cmp("1"), Ordering::Equal);

        let compa_decimal1 = CompaDecimal::from("1").unwrap();
        assert_eq!(compa_decimal1.cmp("0"), Ordering::Greater);

    }
}