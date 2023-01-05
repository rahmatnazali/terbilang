use terbilang;

#[test]
fn test_zero() {
    assert_eq!(terbilang::from(0), String::from("nol"));
}

#[test]
fn test_basic_number_1() {
    assert_eq!(terbilang::from(8), String::from("delapan"));
}

#[test]
fn test_basic_number_2() {
    assert_eq!(terbilang::from(10), String::from("sepuluh"));
}

#[test]
fn test_basic_number_3() {
    assert_eq!(terbilang::from(11), String::from("sebelas"));
}

#[test]
fn test_tens_1() {
    assert_eq!(terbilang::from(12), String::from("dua belas"));
}

#[test]
fn test_tens_2() {
    assert_eq!(terbilang::from(13), String::from("tiga belas"));
}

#[test]
fn test_tens_3() {
    assert_eq!(terbilang::from(19), String::from("sembilan belas"));
}

#[test]
fn test_tens_4() {
    assert_eq!(terbilang::from(20), String::from("dua puluh"));
}

#[test]
fn test_tens_5() {
    assert_eq!(terbilang::from(21), String::from("dua puluh satu"));
}