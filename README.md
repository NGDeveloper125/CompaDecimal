# CompaDecimal

CompaDecimal is a Rust library (crate) that introduces a new decimal system designed to represent large numbers using a compact character set. By combining numbers, letters, and symbols, CompaDecimal allows for high-value numbers to be represented in a smaller number of characters, making it efficient for storage and transmission.

This library is open-source and welcomes contributions, feedback, and suggestions from the community. An extended version of this crate is planned, which will include support for arbitrarily large numbers using BigUint.

## Project Goals

The primary goal of CompaDecimal is to create a compact and efficient decimal system that:

- Represents large numbers in fewer characters by utilizing a custom character set.
- Supports multiple numeric types, including unsigned integers (u8, u16, u32, u64, u128).
- Provides conversion utilities between standard decimal numbers and the CompaDecimal format.
- Ensures extensibility for future support of BigUint for arbitrarily large numbers.

## Features

### 1. Custom Character Set

CompaDecimal uses a custom character set that includes:
- Numbers
- Uppercase and lowercase letters
- Special characters

This character set allows for a base-95 numeral system, enabling compact representation of large numbers.

### 2. Core Functions

The library provides the following core functions:

#### `decimal_to_compa<T>(num: T) -> String`
- Converts a standard decimal number into a CompaDecimal string.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
- Example:
  ```rust
  let num: u32 = 12345;
  let compa = decimal_to_compa(num);
  println!("CompaDecimal: {}", compa);
  ```

#### `compa_to_decimal<T>(compa: &str) -> Result<T, CompaDecimalError>`
- Converts a CompaDecimal string back into a standard decimal number.
- Supports unsigned integer types (u8, u16, u32, u64, u128).
- Example:
  ```rust
  let compa = "A1";
  let decimal: u32 = compa_to_decimal(compa).unwrap();
  println!("Decimal: {}", decimal);
  ```

#### `add_one(current: String) -> Result<String, Error>`
- Increments a CompaDecimal string by one.
- Example:
  ```rust
  let current = "A1".to_string();
  let incremented = add_one(current).unwrap();
  println!("Incremented CompaDecimal: {}", incremented);
  ```

#### `get_next(current: char) -> Result<char, Error>`
- Retrieves the next character in the CompaDecimal character set.
- Example: [Example code here]

### CompaDecimal Struct

A struct that encapsulates a CompaDecimal value and provides methods for conversion and manipulation.
- Example: [Example code here]

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

## Examples

### Convert Decimal to CompaDecimal
[Example code here]

### Convert CompaDecimal to Decimal
[Example code here]

### Increment a CompaDecimal Value
[Example code here]

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