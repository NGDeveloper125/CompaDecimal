# compa_decimal

A compact and efficient decimal system using a custom character set for representing large numbers in fewer characters.

---

## Features

- **Custom Character Set:** Uses numbers, uppercase and lowercase letters, and special characters for a base-95 numeral system.
- **Supports Multiple Numeric Types:** Easily convert between unsigned integers (`u8`, `u16`, `u32`, `u64`, `u128`) and the `CompaDecimal` format.
- **Conversion Utilities:** Convert between standard decimal numbers and the compact `CompaDecimal` representation.
- **Extensible:** Designed for future support of arbitrarily large numbers.

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
compa_decimal = "0.1.1"
```

Import in your Rust code:

```rust
use compa_decimal::CompaDecimal;
```

---

## Examples

### Creating a New CompaDecimal

```rust
let compa = CompaDecimal::new();
assert_eq!(compa, "0");
```

### Parsing from &str
```rust
let compa: CompaDecimal = "123asd".parse().unwrap();
assert_eq!(compa, "123asd");
```

### Decimal to CompaDecimal

```rust
let compa = CompaDecimal::decimal_to_compa::<u64>(123456789).unwrap();
assert_eq!(compa, "1LY7VK");
```

### Get Value

```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
let compa_value = compa.get_value();
assert_eq!(compa_value, "1LY7VK");
```

### Increment and Decrement

```rust
let compa = "A1".parse::<CompaDecimal>().unwrap();
let increased = compa.plus_one().unwrap();
assert_eq!(increased.get_value(), "A2");

let decreased = increased.minus_one().unwrap();
assert_eq!(decreased.get_value(), "A1");
```

### Add and Subtract

```rust
let compa = "ASr35".parse::<CompaDecimal>().unwrap();
let new_compa = compa.add("as1Ad4").unwrap();
assert_eq!(new_compa.get_value(), "axswF9");

let subtracted = new_compa.subtract("as1Ad4").unwrap();
assert_eq!(subtracted.get_value(), "ASr35");
```

### Compare

```rust
use std::cmp::Ordering;

let compa = "axswF9".parse::<CompaDecimal>().unwrap();
assert_eq!(compa.cmp_str("axswF8").unwrap(), Ordering::Greater);
```

### Length

```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
assert_eq!(compa.len(), 6);
```

### Convert Back to Decimal

```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
let decimal: u64 = compa.to_decimal().unwrap();
assert_eq!(decimal, 123456789);
```

### Compare to string

You can compare a `CompaDecimal` directly with a string:

```rust
let compa = "123asd".parse::<CompaDecimal>().unwrap();
assert_eq!(compa, "123asd");
assert_ne!(compa, "not_equal");
```

### Attempt to convert string to `CompaDecimal`

You can use `TryFrom` to create a `CompaDecimal` from a string, which returns a `Result`:

```rust
use std::convert::TryFrom;

let compa = CompaDecimal::try_from("123asd").unwrap();
assert_eq!(compa, "123asd");

// This will return an error if the string contains invalid characters
let invalid = CompaDecimal::try_from("123asd£");
assert!(invalid.is_err());
```

You can also use `try_into` for ergonomic conversion:

```rust
use std::convert::TryInto;

let compa: CompaDecimal = "123asd".try_into().unwrap();
assert_eq!(compa, "123asd");
```

---

## Error Handling

Most methods return a `Result` and will return an error if:
- The input contains invalid characters.
- The operation would result in a negative value.
- The value is too large for the target integer type.

---

## License

Licensed under MIT OR Apache-2.0.