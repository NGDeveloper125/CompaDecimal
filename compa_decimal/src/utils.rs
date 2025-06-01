pub fn get_compa_digits() -> Vec<char> {
    "0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!\"#$%&'()*+,-./:;<=>?@[\\]^_`|}{~".chars().collect()
}

pub fn valid_str(string: &str) -> bool {
    string.chars().all(|ch| get_compa_digits().contains(&ch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_str_test() {
        assert!(valid_str("abc"));
        assert!(valid_str("ABC"));
        assert!(valid_str("123"));
        assert!(!valid_str("£"));
        assert!(!valid_str("¬"));
        assert!(!valid_str("カタカナ"));
        assert!(!valid_str("片"));
    }
}
