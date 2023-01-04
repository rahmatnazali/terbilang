pub fn terbilang(number: usize) -> String {
    todo!("return the terbilang result");
}

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
