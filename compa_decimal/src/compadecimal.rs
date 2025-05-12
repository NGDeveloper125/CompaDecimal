use num::{PrimInt, Unsigned};


struct CompaDecimal {
    value: String 
}

impl CompaDecimal {
    fn new() -> CompaDecimal {
        CompaDecimal { 
            value: "1".to_string() 
        }
    }

    fn from(value: &str) -> Result<CompaDecimal, String> {
        Ok(CompaDecimal { 
            value: value.to_string() 
        })
    }

    fn add_one(&mut self) {
        let mut digits: Vec<char> = self.value.chars().collect();
        let digits_len: usize = digits.len() - 1;

        for i in 1..(digits_len + 1) {
            let digits_len = &digits.len() - i;
            let updated_value = get_next(&digits[digits_len]);
    
            match updated_value {
                '0' => {
                    digits[&digits_len - i] = '0';
                },
                _ => {
                    digits[&digits_len - i] = updated_value;
                    self.value = digits.into_iter().collect::<String>();
                    return;
                }
            }
        }
        digits.insert(0, '1');
        self.value = digits.into_iter().collect::<String>();
            
    }

    pub fn decimal_to_compa<T>(mut num: T) -> CompaDecimal
    where T: PrimInt + Unsigned {
        let chars: Vec<char> = get_compa_digits();
        let base = T::from(chars.len()).unwrap();
        let mut result = String::new();

        if num == T::zero() {
            return CompaDecimal::new();
        }

        while num > T::zero() {
            let reminder = (num % base).to_usize().unwrap();
            result.push(chars[reminder]);
            num = num / base;
        }

        CompaDecimal { value: result.chars().rev().collect() }
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
        assert_eq!(compa_decimal1.value, "0");
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

        let compa_decimal1 = CompaDecimal::decimal_to_compa::<u8>(16);
        assert_eq!(compa_decimal1.value, "D");
        let compa_decimal2 = CompaDecimal::decimal_to_compa::<u32>(1329);
        assert_eq!(compa_decimal2.value, "Cb");
        let compa_decimal3 = CompaDecimal::decimal_to_compa::<u64>(27068251);
        assert_eq!(compa_decimal3.value, "LwOa");
        let compa_decimal4 = CompaDecimal::decimal_to_compa::<u128>(340282366920938463463374607431768211455);
        assert_eq!(compa_decimal4.value, "a2o~TWI*I+5G('\\99=ab");

    }
}