mod divider;

pub fn from(number: u32) -> String {
    if number == 0 {
        return String::from("nol");
    } else if number < 12 {
        return solve_basic_number(number)
    } else if number < 99 {
        return solve_tens(number)
    }
    todo!("The number is too high");
}

/// Solve 1 - 11
fn solve_basic_number(number: u32) -> String {
    match number {
        1 => String::from("satu"),
        2 => String::from("dua"),
        3 => String::from("tiga"),
        4 => String::from("empat"),
        5 => String::from("lima"),
        6 => String::from("enam"),
        7 => String::from("tujuh"),
        8 => String::from("delapan"),
        9 => String::from("sembilan"),
        10 => String::from("sepuluh"),
        11 => String::from("sebelas"),
        _ => panic!("Basic number exceeded 11")
    }
}

/// Solve 12 - 99
fn solve_tens(number: u32) -> String {
    let divide_result = divider::DivideResult::from(number, 10);
    if divide_result.head == 1 {
        format!("{} belas", solve_basic_number(divide_result.remainder))
    } else {
        if divide_result.remainder == 0 {
            format!("{} puluh", solve_basic_number(divide_result.head))
        } else {
            format!("{} puluh {}",
                    solve_basic_number(divide_result.head),
                    solve_basic_number(divide_result.remainder)
            )
        }
    }
}

#[cfg(test)]
mod basic_number_tests {
    use super::*;

    #[test]
    fn satu() {
        assert_eq!(solve_basic_number(1), String::from("satu"));
    }

    #[test]
    fn dua() {
        assert_eq!(solve_basic_number(2), String::from("dua"));
    }

    #[test]
    fn tiga() {
        assert_eq!(solve_basic_number(3), String::from("tiga"));
    }

    #[test]
    fn empat() {
        assert_eq!(solve_basic_number(4), String::from("empat"));
    }

    #[test]
    fn lima() {
        assert_eq!(solve_basic_number(5), String::from("lima"));
    }

    #[test]
    fn enam() {
        assert_eq!(solve_basic_number(6), String::from("enam"));
    }

    #[test]
    fn tujuh() {
        assert_eq!(solve_basic_number(7), String::from("tujuh"));
    }

    #[test]
    fn delapan() {
        assert_eq!(solve_basic_number(8), String::from("delapan"));
    }

    #[test]
    fn sembilan() {
        assert_eq!(solve_basic_number(9), String::from("sembilan"));
    }

    #[test]
    fn sepuluh() {
        assert_eq!(solve_basic_number(10), String::from("sepuluh"));
    }

    #[test]
    fn sebelas() {
        assert_eq!(solve_basic_number(11), String::from("sebelas"));
    }
}

#[cfg(test)]
mod teens_test {
    use super::*;

    #[test]
    fn dua_belas() {
        assert_eq!(solve_tens(12), String::from("dua belas"));
    }

    #[test]
    fn sembilan_belas() {
        assert_eq!(solve_tens(19), String::from("sembilan belas"));
    }
}
