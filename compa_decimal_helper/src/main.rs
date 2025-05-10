use compa_decimal::add_one;

fn main() {
    let mut a = String::from("0"); 
    let loop_len = 100000000000000000000000000000000000; 
    let mut i: i128 = 1;
    while i < loop_len {
        a = add_one(a).unwrap();
        println!("{} - {}", i, a);
        i += 1;
    }
}
