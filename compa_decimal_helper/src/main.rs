use compa_decimal::{add_one, biguint_to_compa, decimal_to_compa};
use num_bigint::BigUint;
use rand::Rng;

fn main() {
    //println!("{}", decimal_to_compa::<u64>(27068251));
    let size = 100;
    let n = generated_random_biguint(size);
    let m = biguint_to_compa(n);
    let m_size = m.len();
    println!("a number with a {} digit goes down to {}", size, m_size);
}                                           

fn generated_random_biguint(digits: usize) -> BigUint {
    let mut rng = rand::rng();
    let mut number = String::new();

    number.push(rng.random_range(1..10).to_string().chars().next().unwrap());

    for _ in 1..digits {
        number.push(rng.random_range(1..10).to_string().chars().next().unwrap());
    }

    BigUint::parse_bytes(number.as_bytes(), 10).unwrap()
}