mod divider;

/// Converts u32 number to said Indonesian
///
/// ```
/// use yaterbilang as terbilang;
///
/// Examples:
/// assert_eq!(terbilang::from(0), "nol");
/// assert_eq!(terbilang::from(11), "sebelas");
/// assert_eq!(terbilang::from(32), "tiga puluh dua");
/// assert_eq!(terbilang::from(998), "sembilan ratus sembilan puluh delapan");
/// ```
///
pub fn from(number: u32) -> String {
    if number == 0 {
        return String::from("nol");
    } else if number < 10 {
        return solve_basic_number(number)
    } else if number < 100 {
        return solve_tens(number)
    } else if number < 1000 {
        return solve_hundreds(number)
    } else if number < 10000 {
        return solve_thousand(number)
    }
    todo!("The number is too high");
}

/// Joins list of string to a single string
fn join_string(list: Vec<String>) -> String {
    let joined_string = list.join(" ");
    let trimmed_string = joined_string.trim();
    trimmed_string.to_string()
}

/// Solve 1 - 9
fn solve_basic_number(number: u32) -> String {
    match number {
        0 => String::from(""),
        1 => String::from("satu"),
        2 => String::from("dua"),
        3 => String::from("tiga"),
        4 => String::from("empat"),
        5 => String::from("lima"),
        6 => String::from("enam"),
        7 => String::from("tujuh"),
        8 => String::from("delapan"),
        9 => String::from("sembilan"),
        _ => panic!("Basic number exceeded 9")
    }
}

/// Solve 12 - 99
fn solve_tens(number: u32) -> String {
    let divide_result = divider::DivideResult::from(number, 10);
    if divide_result.head == 1 {
        if divide_result.remainder == 0 {
            String::from("sepuluh")
        } else if divide_result.remainder == 1 {
            String::from("sebelas")
        } else {
            format!("{} belas", solve_basic_number(divide_result.remainder))
        }
    } else {
        if divide_result.head == 0 {
            solve_basic_number(divide_result.remainder)
        } else {
            join_string(vec![
                format!("{} puluh", solve_basic_number(divide_result.head)),
                solve_basic_number(divide_result.remainder),
            ])
        }
    }
}

/// Solve 100 - 999
fn solve_hundreds(number: u32) -> String {
    let divide_result = divider::DivideResult::from(number, 100);
    if divide_result.head == 0 {
        solve_tens(divide_result.remainder)
    } else if divide_result.head == 1 {
        join_string(vec![
            String::from("seratus"),
            solve_tens(divide_result.remainder),
        ])
    } else {
        join_string(vec![
            format!("{} ratus", solve_basic_number(divide_result.head)),
            solve_tens(divide_result.remainder),
        ])
    }
}

/// Solve 1000 - 9.999
fn solve_thousand(number: u32) -> String {
    let divide_result = divider::DivideResult::from(number, 1000);
    if divide_result.head == 0 {
        solve_hundreds(divide_result.remainder)
    } else if divide_result.head == 1 {
        join_string(vec![
            String::from("seribu"),
            solve_hundreds(divide_result.remainder),
        ])
    } else {
        join_string(vec![
            format!("{} ribu", solve_basic_number(divide_result.head)),
            solve_hundreds(divide_result.remainder),
        ])
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
