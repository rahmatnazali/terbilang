use terbilang;

#[test]
fn test_zero() {
    assert_eq!(terbilang::from(0), String::from("nol"));
}

#[test]
fn test_basic_number_8() {
    assert_eq!(terbilang::from(8), String::from("delapan"));
}

#[test]
fn test_basic_number_10() {
    assert_eq!(terbilang::from(10), String::from("sepuluh"));
}

#[test]
fn test_basic_number_11() {
    assert_eq!(terbilang::from(11), String::from("sebelas"));
}

#[test]
fn test_teens_12() {
    assert_eq!(terbilang::from(12), String::from("dua belas"));
}

#[test]
fn test_teens_13() {
    assert_eq!(terbilang::from(13), String::from("tiga belas"));
}

#[test]
fn test_teens_19() {
    assert_eq!(terbilang::from(19), String::from("sembilan belas"));
}

#[test]
fn test_tens_20() {
    assert_eq!(terbilang::from(20), String::from("dua puluh"));
}

#[test]
fn test_tens_21() {
    assert_eq!(terbilang::from(21), String::from("dua puluh satu"));
}

#[test]
fn test_tens_99() {
    assert_eq!(terbilang::from(99), String::from("sembilan puluh sembilan"));
}

#[test]
fn test_hundred_100() {
    assert_eq!(terbilang::from(100), String::from("seratus"));
}

#[test]
fn test_hundred_101() {
    assert_eq!(terbilang::from(101), String::from("seratus satu"));
}

#[test]
fn test_hundred_110() {
    assert_eq!(terbilang::from(110), String::from("seratus sepuluh"));
}