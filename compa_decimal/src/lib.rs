use std::{collections::HashMap, fmt::Error};


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
    a
}

pub fn get_next(current: char) -> Result<char, Error> {
    let map = generate_set();
    //let digits: Vec<char> = current.chars().collect();
    //let last_digit = digits.last().unwrap();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_test() {
        assert_eq!(get_next('0').unwrap(), '1');
        assert_eq!(get_next('9').unwrap(), 'A');
        assert_eq!(get_next('A').unwrap(), 'a');
        assert_eq!(get_next('z').unwrap(), '+');
    }
}
