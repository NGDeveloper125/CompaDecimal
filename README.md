# CompaDecimal

CompaDecimal is a Rust library (crate) that introduces a new decimal system designed to represent large numbers using a compact character set. By combining numbers, letters, and symbols, CompaDecimal allows for high-value numbers to be represented in a smaller number of characters, making it efficient for storage and transmission.

This library is open-source and welcomes contributions, feedback, and suggestions from the community. An extended version of this crate is planned, which will include support for arbitrarily large numbers using `BigUint`.

---

## Project Goals

The primary goal of CompaDecimal is to create a compact and efficient decimal system that:
- **Represents large numbers in fewer characters** by utilizing a custom character set.
- **Supports multiple numeric types**, including unsigned integers (`u8`, `u16`, `u32`, `u64`, `u128`).
- **Provides conversion utilities** between standard decimal numbers and the CompaDecimal format.
- **Ensures extensibility** for future support of `BigUint` for arbitrarily large numbers.

---

## Features

### 1. Custom Character Set

CompaDecimal uses a custom character set that includes:
- Numbers
- Uppercase and lowercase letters
- Special characters

This character set allows for a base-95 numeral system, enabling compact representation of large numbers.

### 2. Core Functions

The library provides the following core functions:

#### `CompaDecimal::new() -> CompaDecimal`
- Creates a `CompaDecimal` object with the value set to `String::from("0")`.
- **Example**:
```rust
let compa = CompaDecimal::new();
assert_eq(compa.value, "0".to_string());
```

#### `CompaDecimal::from(value: &str) -> Result<CompaDecimal, CompaDecimalError>`
- Attempting to create a `CompaDecimal` object with the value set to the `value` parameter.
- **Example**:
```rust
let compa = CompaDecimal::from("123asd").unwrap();
assert_eq(compa.value, "123asd".to_string());
```

#### `CompaDecimal::decimal_to_compa<T>(num: T) -> Result<CompaDecimal, CompaDecimalError>`
- Attempting to converts a standard decimal number into a `CompaDecimal` value.
- Supports unsigned integer types (`u8`, `u16`, `u32`, `u64`, `u128`).
- **Example**:
```rust
let compa = CompaDecimal::decimal_to_compa:<u64>:(123456789).unwrap();
assert_eq(compa.value, "1LY7VK".to_string());
```

#### `plus_one(&mut self)`
- Increments the CompaDecimal value by one.
```rust
let mut compa = CompaDecimal::from("A1").unwrap();
compa.plus_one();
assert_eq(compa.value, "A2".to_string());
```

#### `minus_one(&self)`
- Attempts to decrease the CompaDecimal value by one.
```rust
let compa = CompaDecimal::from("A1").unwrap();
let decreased = compa.minus_one().unwrap();
assert_eq(compa.value, "A".to_string());
```

#### `increase_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to increases the CompaDecimal value by a specified amount.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
```rust
let compa = CompaDecimal::from("1LY7VK").unwrap();
let increased = compa.increase_by::<u32>(1234).unwrap();
assert_eq(increased.value, "1LY7$Q".to_string());
```

#### `decrease_by<T>(&self, amount: T) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to decreases the CompaDecimal value by a specified amount.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
```rust
let compa = CompaDecimal::from("1LY7VK").unwrap();
let decreased = compa.decrease_by::<u32>(1234).unwrap();
assert_eq(increased.value, "1LY7oE".to_string());
```

#### `add(&self, additional_value: &str) -> CompaDecimal`
- Add the additional value to the CompaDecimal value.
```rust
let compa = CompaDecimal::from("ASr35").unwrap();
let new_compa = compa.add("as1Ad4");
assert_eq(new_compa.value, "axswF9".to_string());
```

#### `subtract(&self, subtrahend: &str) -> Result<CompaDecimal, CompaDecimalError>`
- Attempts to subract the subtrahend value from the CompaDecimal value.
- Returns an error if the subtrahend is bigger than the CompaDecimal value.
```rust
let compa = CompaDecimal::from("axswF9").unwrap();
let new_compa = compa.subtract("as1Ad4").unwrap();
assert_eq(new_compa.value, "ASr35".to_string());
```

#### `len(&self) -> usize`
- Returns the length of the CompaDecimal value.
```rust
let compa = CompaDecimal::from("1LY7VK").unwrap();
assert_eq(compa.len(), 6);
```

#### `to_decimal<T>(&self) -> Result<T, CompaDecimalError>`
- Attempts to converts a CompaDecimal value back into a standard decimal number.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
```rust
let compa = CompaDecimal::new("1LY7VK".to_string());
let decimal = compa.to_decimal::<u64>().unwrap();
assert_eq(123456789, decimal);
```

## Planned Features

An extended version of this crate is in development, which will include:
- Support for BigUint: Allowing representation and manipulation of arbitrarily large numbers.
- Additional utilities for advanced operations on CompaDecimal values.

## Installation

Add the following to your Cargo.toml to include CompaDecimal in your project:

```toml
[dependencies]
compadecimal = "0.1.0"
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