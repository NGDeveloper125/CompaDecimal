# CompaDecimal

CompaDecimal is a Rust library (crate) that introduces a new decimal system designed to represent large numbers using a compact character set. <br>
By combining numbers, letters, and symbols, CompaDecimal allows for high-value numbers to be represented in a smaller number of characters, making it efficient for storage and transmission.

This library is open-source and welcomes contributions, feedback, and suggestions from the community. An extended version of this crate is planned, which will include support for arbitrarily large numbers using `BigUint`.

---

## Project Goals

The primary goal of CompaDecimal is to create a compact and efficient decimal system that:
- **Represents large numbers in fewer characters** by utilizing a custom character set.
- **Supports multiple numeric types**, including unsigned integers (`u8`, `u16`, `u32`, `u64`, `u128`).
- **Provides conversion utilities** between standard decimal numbers and the `CompaDecimal` format.
- **Ensures extensibility** for future support of `BigUint` for arbitrarily large numbers.

---

## Features

### 1. Custom Character Set

`CompaDecimal` uses a custom character set that includes:
- Numbers
- Uppercase and lowercase letters
- Special characters

This character set allows for a base-95 numeral system, enabling compact representation of large numbers.

### creating and casting examples 
The library provides the following core functions and trait implementations:

#### `CompaDecimal::new() -> CompaDecimal`
- Creates a `CompaDecimal` object with the value set to `String::from("0")`.
- **Example**:
```rust
let compa = CompaDecimal::new();
assert_eq(compa, "0");
```

#### `FromStr trait`
- Using the FromStr trait to parse an string to `CompaDecimal` object with the string as the `CompaDecimal` object's value.
- Returns an error if string contains invalid characters.
- **Example 1**:
```rust
let compa: CompaDecimal = "123asd".parse().unwrap();
assert_eq(compa, "123asd");
```
- **Example 2**:
```rust
let compa = "123asd".parse::<CompaDecimal>().unwrap();
assert_eq(compa, "123asd");
```

#### `CompaDecimal::decimal_to_compa<T>(num: T) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to convert a standard decimal number into a `CompaDecimal` value.
- Supports unsigned integer types (`u8`, `u16`, `u32`, `u64`, `u128`).
- **Example**:
```rust
let compa = CompaDecimal::decimal_to_compa::<u64>(123456789).unwrap();
assert_eq(compa, "1LY7VK".);
```
**Note**:
This function returns a Result but under normal circumstances, it should never return an error. An error would only occur if the internal state of the `CompaDecimal` is invalid (e.g., contains non-compa digits), which should not happen if the object was constructed correctly.

#### `get_value(&self) -> &str`
- Returns the **CompaDecimal** object's value.
```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
let compa_value = compa.get_value();
assert_eq(compa_value, "1LY7VK");
```

#### `plus_one(self) -> Result<CompaDecimal, CompaDecimalError>`
- Increments the `CompaDecimal` value by one and return a result with a new object with the updated value. 
```rust
- 
let compa = "A1".parse::<CompaDecimal>().unwrap();
let increased = compa.plus_one().unwrap();
assert_eq(increased, "A2");
```
**Note**:
This function returns a Result but under normal circumstances, it should never return an error. An error would only occur if the internal state of the `CompaDecimal` is invalid (e.g., contains non-compa digits), which should not happen if the object was constructed correctly.

#### `minus_one(&self) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to decrease the `CompaDecimal` value by one and returns a new object with the updated value.
- Returns an error if the value is zero (cannot decrement below zero).
```rust
let compa = "A1".parse::<CompaDecimal>().unwrap();
let decreased = compa.minus_one().unwrap();
assert_eq(decreased, "A");
```

#### `increase_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to increase the `CompaDecimal` value by a specified amount.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
let increased = compa.increase_by::<u32>(1234).unwrap();
assert_eq(increased, "1LY7$Q");
```
**Note**:
This function returns a Result but under normal circumstances, it should never return an error. An error would only occur if the internal state of the `CompaDecimal` is invalid (e.g., contains non-compa digits), which should not happen if the object was constructed correctly.

#### `decrease_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to decrease the `CompaDecimal` value by a specified amount.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
- Returns an error if the result value is less than zero (`CompaDecimal` cannot hold negative value).
```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
let decreased = compa.decrease_by::<u32>(1234).unwrap();
assert_eq(decreased, "1LY7oE");
```

#### `add(&self, additional_value: &str) -> Result<CompaDecimal, CompaDecimalError>`
- Adds the `additional_value` to the `CompaDecimal` value.
- Returns an error if the `additional_value` contains invalid characters.
```rust
let compa = "ASr35".parse::<CompaDecimal>().unwrap();
let new_compa = compa.add("as1Ad4");
assert_eq(new_compa, "axswF9");
```

#### `subtract(&self, subtrahend: &str) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to subtract the subtrahend value from the `CompaDecimal` value.
- Returns an error if the subtrahend is greater than the `CompaDecimal` value.
```rust
let compa = "axswF9".parse::<CompaDecimal>().unwrap();
let new_compa = compa.subtract("as1Ad4").unwrap();
assert_eq(new_compa, "ASr35");
```

#### `cmp_str(&self, comparand: &str) -> Result<std::cmp::Ordering, CompaDecimalError>`
- Attempts to compare the comparand string value to the `CompaDecimal` value.
- Returns an Ordering.
- Returns an error if the comparand contain invalid characters.
```rust
let compa = "axswF9".parse::<CompaDecimal>().unwrap();
assert_eq(compa.cmp_str("axswF8"), Ordering::Less);
```

#### `len(&self) -> usize`
- Returns the length of the `CompaDecimal` value.
```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
assert_eq(compa.len(), 6);
```

#### `to_decimal<T>(&self) -> Result<T, CompaDecimalError>`
- Attempts to convert a `CompaDecimal` value back into a standard decimal number.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
- Returns CompaDecimalError if value is too big for the integer type. 
```rust
let compa = "1LY7VK".parse::<CompaDecimal>().unwrap();
let decimal = compa.to_decimal::<u64>().unwrap();
assert_eq(123456789, decimal);
```

#### `eq(&self, other: &&str) -> bool` (PartialEq<&str> trait)
- Allows direct comparison between a `CompaDecimal` object and a string.
- **Example**:
```rust
let compa = "123asd".parse::<CompaDecimal>().unwrap();
assert_eq!(compa, "123asd");
assert_ne!(compa, "not_equal");
```

#### `try_from(value: &str) -> Result<Self, Self::Error>` (TryFrom<&str> trait)
- Attempts to create a `CompaDecimal` object from a string, returning a `Result`.
- Returns an error if the string contains invalid characters.
- **Example**:
```rust
use std::convert::TryFrom;

let compa = CompaDecimal::try_from("123asd").unwrap();
assert_eq!(compa, "123asd");

// This will return an error if the string contains invalid characters
let invalid = CompaDecimal::try_from("123asd£");
assert!(invalid.is_err());
```

## Planned Features

An extended version of this crate is in development, which will include:
- Support for `BigUint`: Allowing representation and manipulation of arbitrarily large numbers.
- Additional utilities for advanced operations on `CompaDecimal` values.

## Installation

Add the following to your Cargo.toml to include CompaDecimal in your project:

```toml
[dependencies]
compadecimal = "0.1.1"
```

Then, import the crate in your Rust code:

```rust
use compadecimal::*;
```


## Contributing

CompaDecimal is an open-source project, and contributions are welcome! Whether it's reporting bugs, suggesting features, or submitting pull requests, your input is highly valued.

### How to Contribute

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with a detailed description of your changes.

## Feedback

If you have any feedback or suggestions, feel free to open an issue on the GitHub repository.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

Thank you to the Rust community for providing the tools and libraries that made this project possible. Special thanks to contributors for their valuable input and support.

Start using CompaDecimal today and make your large numbers compact and efficient! 🚀