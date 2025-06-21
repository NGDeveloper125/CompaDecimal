use compa_decimal::*;
use std::cmp::Ordering;

#[test]
fn try_from_test() {
    let compa_decimal1 = CompaDecimal::try_from("123asd").unwrap();
    assert_eq!(compa_decimal1, "123asd");

    let compa_decimal1 = CompaDecimal::try_from("123asd£");
    assert!(compa_decimal1.is_err());
}

#[test]
fn try_into() {
    let compa_decimal1: CompaDecimal = "123asd".try_into().unwrap();
    assert_eq!(compa_decimal1, "123asd");

    let compa_decimal1: Result<CompaDecimal, CompaDecimalError> = "123asd£".try_into();
    assert!(compa_decimal1.is_err());
}

#[test]
fn eq_test() {
    let compa_decimal1: CompaDecimal = "abc123".parse().unwrap();
    assert_eq!(compa_decimal1, "abc123");

    let compa_decimal2: CompaDecimal = "abc1234".parse().unwrap();
    assert_ne!(compa_decimal2, "abc123");
}

#[test]
fn plus_one_test() {
    let compa_decimal1: CompaDecimal = "0".parse().unwrap();
    let compa_decimal1 = compa_decimal1.plus_one().unwrap();
    assert_eq!(compa_decimal1, "1");
    let compa_decimal1 = compa_decimal1.plus_one().unwrap();
    assert_eq!(compa_decimal1, "2");
    let compa_decimal2: CompaDecimal = "9".parse().unwrap();
    let compa_decimal2 = compa_decimal2.plus_one().unwrap();
    assert_eq!(compa_decimal2, "A");
    let compa_decimal3: CompaDecimal = "z".parse().unwrap();
    let compa_decimal3 = compa_decimal3.plus_one().unwrap();
    assert_eq!(compa_decimal3, "!");
    let compa_decimal4: CompaDecimal = "10".parse().unwrap();
    let compa_decimal4 = compa_decimal4.plus_one().unwrap();
    assert_eq!(compa_decimal4, "11");
    let compa_decimal5: CompaDecimal = "19".parse().unwrap();
    let compa_decimal5 = compa_decimal5.plus_one().unwrap();
    assert_eq!(compa_decimal5, "1A");
    let compa_decimal6: CompaDecimal = "1z".parse().unwrap();
    let compa_decimal6 = compa_decimal6.plus_one().unwrap();
    assert_eq!(compa_decimal6, "1!");
    let compa_decimal7: CompaDecimal = "1~".parse().unwrap();
    let compa_decimal7 = compa_decimal7.plus_one().unwrap();
    assert_eq!(compa_decimal7, "20");
}

#[test]
fn miuns_one_test() {
    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal1 = compa_decimal1.minus_one().unwrap();
    assert_eq!(compa_decimal1, "0");
    let compa_decimal2: CompaDecimal = "A".parse().unwrap();
    let compa_decimal2 = compa_decimal2.minus_one().unwrap();
    assert_eq!(compa_decimal2, "9");
    let compa_decimal3: CompaDecimal = "!".parse().unwrap();
    let compa_decimal3 = compa_decimal3.minus_one().unwrap();
    assert_eq!(compa_decimal3, "z");
    let compa_decimal4: CompaDecimal = "11".parse().unwrap();
    let compa_decimal4 = compa_decimal4.minus_one().unwrap();
    assert_eq!(compa_decimal4, "10");
    let compa_decimal5: CompaDecimal = "1A".parse().unwrap();
    let compa_decimal5 = compa_decimal5.minus_one().unwrap();
    assert_eq!(compa_decimal5, "19");
    let compa_decimal6: CompaDecimal = "1z".parse().unwrap();
    let compa_decimal6 = compa_decimal6.minus_one().unwrap();
    assert_eq!(compa_decimal6, "1Z");
    let compa_decimal7: CompaDecimal = "20".parse().unwrap();
    let compa_decimal7 = compa_decimal7.minus_one().unwrap();
    assert_eq!(compa_decimal7, "1~");
    let compa_decimal7: CompaDecimal = "10".parse().unwrap();
    let compa_decimal7 = compa_decimal7.minus_one().unwrap();
    assert_eq!(compa_decimal7, "~");
}

#[test]
fn decimal_to_compa_test() {
    let compa_decimal1 = CompaDecimal::decimal_to_compa::<u8>(16).unwrap();
    assert_eq!(compa_decimal1, "D");
    let compa_decimal2 = CompaDecimal::decimal_to_compa::<u32>(1329).unwrap();
    assert_eq!(compa_decimal2, "b~");
    let compa_decimal3 = CompaDecimal::decimal_to_compa::<u64>(27068251).unwrap();
    assert_eq!(compa_decimal3, "kWg}");
    let compa_decimal4 =
        CompaDecimal::decimal_to_compa::<u128>(340282366920938463463374607431768211455).unwrap();
    assert_eq!(compa_decimal4, "91\"<n.hl48T!YkTkA?1Z");
}

#[test]
fn to_decimal_test() {
    let compa_decimal1: CompaDecimal = "D".parse().unwrap();
    assert_eq!(compa_decimal1.to_decimal::<u8>().unwrap(), 16);

    let compa_decimal2: CompaDecimal = "Cb".parse().unwrap();
    assert_eq!(compa_decimal2.to_decimal::<u32>().unwrap(), 1343);

    let compa_decimal3: CompaDecimal = "LwOa".parse().unwrap();
    assert_eq!(compa_decimal3.to_decimal::<u64>().unwrap(), 27935996);

    let compa_decimal4: CompaDecimal = "a2o~TWI*I+5G('".parse().unwrap();
    assert_eq!(
        compa_decimal4.to_decimal::<u128>().unwrap(),
        565984502558084335516371423
    );
}

#[test]
fn len_test() {
    let compa_decimal1: CompaDecimal = "123".parse().unwrap();
    assert_eq!(compa_decimal1.len(), 3);
}

#[test]
fn increase_by_test() {
    let mut compa_decimal1 = CompaDecimal::new();
    compa_decimal1 = compa_decimal1.increase_by::<u8>(1).unwrap();
    assert_eq!(compa_decimal1, "1");

    let mut compa_decimal2 = CompaDecimal::new();
    compa_decimal2 = compa_decimal2.increase_by::<u32>(1234).unwrap();
    assert_eq!(compa_decimal2, "B~");

    let mut compa_decimal3 = CompaDecimal::new();
    compa_decimal3 = compa_decimal3.increase_by::<u64>(1234567).unwrap();
    assert_eq!(compa_decimal3, "1p.Q");

    let mut compa_decimal4 = CompaDecimal::new();
    compa_decimal4 = compa_decimal4.increase_by::<u128>(1234556778785).unwrap();
    assert_eq!(compa_decimal4, "1#VaH@U");
}

#[test]
fn decrease_by_test() {
    let mut compa_decimal1: CompaDecimal = "1".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u8>(1).unwrap();
    assert_eq!(compa_decimal1, "0");

    let mut compa_decimal1: CompaDecimal = "bB".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u32>(1234).unwrap();
    assert_eq!(compa_decimal1, "b");

    let mut compa_decimal1: CompaDecimal = "1r&$".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u64>(1234567).unwrap();
    assert_eq!(compa_decimal1, "3^g");

    let mut compa_decimal1: CompaDecimal = "1-Fq}q3".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u128>(1234556778785).unwrap();
    assert_eq!(compa_decimal1, "9\"L%WT");

    let mut compa_decimal1: CompaDecimal = "1-Fq}q3".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u8>(1).unwrap();
    assert_eq!(compa_decimal1, "1-Fq}q2");

    let mut compa_decimal1: CompaDecimal = "1-Fq}q3".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u16>(100).unwrap();
    assert_eq!(compa_decimal1, "1-Fq}p ");

    let mut compa_decimal1: CompaDecimal = "1-Fq}q3".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u32>(2395784).unwrap();
    assert_eq!(compa_decimal1, "1-Fpc~H");

    let mut compa_decimal1: CompaDecimal = "1-Fq}q3".parse().unwrap();
    compa_decimal1 = compa_decimal1.decrease_by::<u128>(234897382497).unwrap();
    assert_eq!(compa_decimal1, "1q>uFz*");
}

#[test]
fn add_test() {
    let compa_decimal1 = CompaDecimal::new();
    let compa_decimal1 = compa_decimal1.add("1").unwrap();
    assert_eq!(compa_decimal1, "1");

    let compa_decimal1 = CompaDecimal::new();
    let compa_decimal1 = compa_decimal1.add("1AWS").unwrap();
    assert_eq!(compa_decimal1, "1AWS");

    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal1 = compa_decimal1.add("1").unwrap();
    assert_eq!(compa_decimal1, "2");

    let compa_decimal1: CompaDecimal = "aAswf".parse().unwrap();
    let compa_decimal1 = compa_decimal1.add("AsdgrW11").unwrap();
    assert_eq!(compa_decimal1, "AsdMX7XG");
}

#[test]
fn subtract_test() {
    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal1 = compa_decimal1.subtract("1").unwrap();
    assert_eq!(compa_decimal1, "0");

    let compa_decimal1: CompaDecimal = "1AWS".parse().unwrap();
    let compa_decimal1 = compa_decimal1.subtract("1AWS").unwrap();
    assert_eq!(compa_decimal1, "0");

    let compa_decimal1: CompaDecimal = "2".parse().unwrap();
    let compa_decimal1 = compa_decimal1.subtract("1").unwrap();
    assert_eq!(compa_decimal1, "1");

    let compa_decimal1: CompaDecimal = "AsdMX7XG".parse().unwrap();
    let compa_decimal1 = compa_decimal1.subtract("AsdgrW11").unwrap();
    assert_eq!(compa_decimal1, "aATwf");

    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal1 = compa_decimal1.subtract("2");
    assert!(compa_decimal1.is_err());
}

#[test]
fn cmp_test() {
    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal2: CompaDecimal = "2".parse().unwrap();
    assert!(compa_decimal1 < compa_decimal2);

    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal2: CompaDecimal = "1".parse().unwrap();
    assert!(compa_decimal1 == compa_decimal2);

    let compa_decimal1: CompaDecimal = "1".parse().unwrap();
    let compa_decimal2: CompaDecimal = "0".parse().unwrap();
    assert!(compa_decimal1 > compa_decimal2);
}

#[test]
fn cmp_str_test() {
    let compa_decimal1: CompaDecimal = "df$fG35SDd".parse().unwrap();
    assert_eq!(
        compa_decimal1.cmp_str("4Dfh4hd").unwrap(),
        Ordering::Greater
    );

    let compa_decimal1: CompaDecimal = "df$fG35SDd".parse().unwrap();
    assert_eq!(
        compa_decimal1.cmp_str("df$fG35SDd").unwrap(),
        Ordering::Equal
    );

    let compa_decimal1: CompaDecimal = "df$fG35SDd".parse().unwrap();
    assert_eq!(
        compa_decimal1.cmp_str("df$fG35SDd$%FDgfd2d").unwrap(),
        Ordering::Less
    );
}
