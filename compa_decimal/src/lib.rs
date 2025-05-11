use std::{collections::HashMap, fmt::Error};

use num::{BigUint, PrimInt, Unsigned, Zero};
use num_traits::ToPrimitive;
mod compadecimal;

fn generate_set() -> HashMap<usize, char> {
    let mut a = HashMap::new();
    a.insert(1, '0');
    a.insert(2, '1');
    a.insert(3, '2');
    a.insert(4, '3');
    a.insert(5, '4');
    a.insert(6, '5');
    a.insert(7, '6');
    a.insert(8, '7');
    a.insert(9, '8');
    a.insert(10, '9');
    a.insert(11, 'A');
    a.insert(12, 'a');
    a.insert(13, 'B');
    a.insert(14, 'b');
    a.insert(15, 'C');
    a.insert(16, 'c');
    a.insert(17, 'D');
    a.insert(18, 'd');
    a.insert(19, 'E');
    a.insert(20, 'e');
    a.insert(21, 'F');
    a.insert(22, 'f');
    a.insert(23, 'G');
    a.insert(24, 'g');
    a.insert(25, 'H');
    a.insert(26, 'h');
    a.insert(27, 'i');
    a.insert(28, 'I');
    a.insert(29, 'J');
    a.insert(30, 'j');
    a.insert(31, 'K');
    a.insert(32, 'k');
    a.insert(33, 'L');
    a.insert(34, 'l');
    a.insert(35, 'M');
    a.insert(36, 'm');
    a.insert(37, 'N');
    a.insert(38, 'n');
    a.insert(39, 'O');
    a.insert(40, 'o');
    a.insert(41, 'P');
    a.insert(42, 'p');
    a.insert(43, 'Q');
    a.insert(44, 'q');
    a.insert(45, 'R');
    a.insert(46, 'r');
    a.insert(47, 'S');
    a.insert(48, 's');
    a.insert(49, 'T');
    a.insert(50, 't');
    a.insert(51, 'U');
    a.insert(52, 'u');
    a.insert(53, 'V');
    a.insert(54, 'v');
    a.insert(55, 'W');
    a.insert(56, 'w');
    a.insert(57, 'X');
    a.insert(58, 'x');
    a.insert(59, 'Y');
    a.insert(60,'y');
    a.insert(61, 'Z');
    a.insert(62, 'z');
    a.insert(63, '!');
    a.insert(64, '\"');
    a.insert(65, '#');
    a.insert(66, '$');
    a.insert(67, '%');
    a.insert(68, '&');
    a.insert(69, '\'');
    a.insert(70, '(');
    a.insert(71, ')');
    a.insert(72, '*');
    a.insert(73, '+');
    a.insert(74, ',');
    a.insert(75, '-');
    a.insert(76, '.');
    a.insert(77, '/');
    a.insert(78, ':');
    a.insert(79, ';');
    a.insert(80, '<');
    a.insert(81, '=');
    a.insert(82, '>');
    a.insert(84, '@');
    a.insert(85, '[');
    a.insert(86, '\\');
    a.insert(87, ']');
    a.insert(88, '^');
    a.insert(89, '_');
    a.insert(91, '`');
    a.insert(92, '|');
    a.insert(93, '}');
    a.insert(94, '{');
    a.insert(95, '~');
    a
    }

fn get_next(current: char) -> Result<char, Error> {
    let map = generate_set();
    return match map.iter()
             .find_map(|(key, &val)| if val == current {Some(key)} else {None}) {
                Some(key) => {
                    match map.get(&(key + 1)) {
                        Some(new_val) => return Ok(*new_val),
                        None => return Ok('+')
                    }},
                None =>  Err(Error)
            }
}

pub fn add_one(current: String) -> Result<String, Error> {
    let mut digits: Vec<char> = current.chars().collect();
    let digits_len = digits.len();
    for i in 1..(digits_len +1) {
     
        let updated_value = match get_next(digits[digits.len() - i]) {
            Ok(val) => val,
            Err(error) => return Err(error)
        };

        match updated_value {
            '+' => {
                digits[digits_len - i] = '0';
            },
            _ => {
                digits[digits_len - i] = updated_value;
                return Ok(digits.into_iter().collect::<String>())
            }
        }
    }
    digits.insert(0, '1');
    return Ok(digits.into_iter().collect::<String>());
}

pub fn decimal_to_compa<T>(mut num: T) -> String
    where T: PrimInt + Unsigned {
    let chars: Vec<char> = "0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!\"#$%&'()*+,-./:;<=>?@[\\]^_`|}{~".chars().collect();
    let base = T::from(chars.len()).unwrap();
    let mut result = String::new();

    if num == T::zero() {
        return "0".to_string();
    }

    while num > T::zero() {
        let reminder = (num % base).to_usize().unwrap();
        result.push(chars[reminder]);
        num = num / base;
    }

    result.chars().rev().collect()
}

pub fn biguint_to_compa(num: BigUint) -> String {
    let chars: Vec<char> = "0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!\"#$%&'()*+,-./:;<=>?@[\\]^_`|}{~".chars().collect();
    let base = BigUint::from(chars.len());
    let mut result = String::new();
    let mut num = num;

    if num.is_zero() {
        return "0".to_string();
    }

    while num > BigUint::zero() {
        let reminder = (&num % &base).to_usize().unwrap();
        result.push(chars[reminder]);
        num /= &base;
    }

    result.chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_test() {
        assert_eq!(get_next('0').unwrap(), '1');
        assert_eq!(get_next('9').unwrap(), 'A');
        assert_eq!(get_next('A').unwrap(), 'a');
        assert_eq!(get_next('~').unwrap(), '+');
    }

    #[test]
    fn add_one_test() {
        assert_eq!(add_one(String::from("0")).unwrap(), "1");
        assert_eq!(add_one(String::from("1")).unwrap(), "2");
        assert_eq!(add_one(String::from("9")).unwrap(), "A");
        assert_eq!(add_one(String::from("z")).unwrap(), "!");
        assert_eq!(add_one(String::from("10")).unwrap(), "11");
        assert_eq!(add_one(String::from("19")).unwrap(), "1A");
        assert_eq!(add_one(String::from("1z")).unwrap(), "1!");
        assert_eq!(add_one(String::from("1~")).unwrap(), "20");
    }

    #[test]
    fn decimal_to_compa_test() {
        assert_eq!(decimal_to_compa::<u64>(27068251), "LwOa");
        assert_eq!(decimal_to_compa::<u128>(340282366920938463463374607431768211455), "a2o~TWI*I+5G('\\99=ab");
    }
}
