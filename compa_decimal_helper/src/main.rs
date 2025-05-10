use compa_decimal::{add_one, decimal_to_compa};

fn main() {
    println!("{}", decimal_to_compa::<u128>(340282366920938463463374607431768211455));
}
