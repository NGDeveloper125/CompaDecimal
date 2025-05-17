use std::any::{type_name, type_name_of_val};

use num::{PrimInt, Unsigned};

#[derive(Debug)]
struct CompaDecimalError {
    error_message: String
}
struct CompaDecimal {
    value: String 
}

impl CompaDecimal {
    fn new() -> CompaDecimal {
        CompaDecimal { 
            value: "0".to_string() 
        }
    }

    fn from(value: &str) -> Result<CompaDecimal, CompaDecimalError> {
        Ok(CompaDecimal { 
            value: value.to_string() 
        })
    }

    fn add_one(&mut self) {
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

    pub fn increase_by<T>(&mut self, amount: T)
    where T: PrimInt + Unsigned {
        let mut looper = T::from(0).unwrap();
        while amount > looper{
            self.add_one();
            looper = looper + T::from(1).unwrap();
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_test() {
        assert_eq!(get_next(&'0'), '1');
        assert_eq!(get_next(&'9'), 'A');
        assert_eq!(get_next(&'A'), 'a');
        assert_eq!(get_next(&'~'), '0');
    }

    #[test]
    fn add_one_test() {
        let mut compa_decimal1 = CompaDecimal::from("0").unwrap();
        compa_decimal1.add_one();
        assert_eq!(compa_decimal1.value, "1");
        compa_decimal1.add_one();
        assert_eq!(compa_decimal1.value, "2");
        let mut compa_decimal2 = CompaDecimal::from("9").unwrap();
        compa_decimal2.add_one();
        assert_eq!(compa_decimal2.value, "A");
        let mut compa_decimal3 = CompaDecimal::from("z").unwrap();
        compa_decimal3.add_one();
        assert_eq!(compa_decimal3.value, "!");
        let mut compa_decimal4 = CompaDecimal::from("10").unwrap();
        compa_decimal4.add_one();
        assert_eq!(compa_decimal4.value, "11");
        let mut compa_decimal5 = CompaDecimal::from("19").unwrap();
        compa_decimal5.add_one();
        assert_eq!(compa_decimal5.value, "1A");
        let mut compa_decimal6 = CompaDecimal::from("1z").unwrap();
        compa_decimal6.add_one();
        assert_eq!(compa_decimal6.value, "1!");
        let mut compa_decimal7 = CompaDecimal::from("1~").unwrap();
        compa_decimal7.add_one();
        assert_eq!(compa_decimal7.value, "20");
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
        compa_decimal1.increase_by::<u8>(1);
        assert_eq!(compa_decimal1.value, "1");

        let mut compa_decimal1 = CompaDecimal::new();
        compa_decimal1.increase_by::<u32>(1234);
        assert_eq!(compa_decimal1.value, "bB");

        let mut compa_decimal1 = CompaDecimal::new();
        compa_decimal1.increase_by::<u64>(1234567);
        assert_eq!(compa_decimal1.value, "1r&$");

        let mut compa_decimal1 = CompaDecimal::new();
        compa_decimal1.increase_by::<u128>(1234556778785);
        assert_eq!(compa_decimal1.value, "1-Fq}q3");
    }
}