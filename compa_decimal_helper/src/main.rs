use compa_decimal::compadecimal::*;
use num_bigint::BigUint;

fn main() {
    // let i = 867550179;
    // let b: u128 = 681028071162207;
    // let result = i + b;

    // let compa_decimal1 = CompaDecimal::from("AsdgrW11").unwrap();
    // let result = compa_decimal1.to_decimal::<u128>().unwrap();

    //let compa_decimal1 = CompaDecimal::decimal_to_compa::<u128>(681028938712386).unwrap();
    // let compa = CompaDecimal::from("1LY7VK").unwrap();
    // let increased = compa.increase_by::<u32>(1234).unwrap();
    let compa_decimal1= CompaDecimal::biguint_to_compa(&BigUint::from(138945729038763748276832u128)).unwrap();
    //let number = compa_decimal1.to_decimal::<u128>().unwrap();
    //let number = compa_decimal1.to_decimal::<u128>().unwrap();
    // compa_decimal1 = compa_decimal1.decrease_by::<u128>(234897382497).unwrap();
    println!("{}", compa_decimal1);
    // let size = 100;
    // let n = generated_random_biguint(size);
    // let m = biguint_to_compa(n);
    // let m_size = m.len();
    // println!("a number with a {} digit goes down to {}", size, m_size);
}

// fn generated_random_biguint(digits: usize) -> BigUint {
//     let mut rng = rand::rng();
//     let mut number = String::new();

//     number.push(rng.random_range(1..10).to_string().chars().next().unwrap());

//     for _ in 1..digits {
//         number.push(rng.random_range(1..10).to_string().chars().next().unwrap());
//     }

//     BigUint::parse_bytes(number.as_bytes(), 10).unwrap() 867550179 + 681028071162207
// }
