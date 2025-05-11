
struct CompaDecimal {
    value: String 
}

impl CompaDecimal {
    fn new() -> CompaDecimal {
        CompaDecimal { 
            value: String::new() 
        }
    }

    fn from(value: &str) -> CompaDecimal {
        CompaDecimal { 
            value: value.to_string() 
        }
    }
}