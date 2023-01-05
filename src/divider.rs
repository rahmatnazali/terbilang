pub struct DivideResult {
    pub head: u32,
    pub remainder: u32
}

impl DivideResult {
    pub fn from(number: u32, denominator: u32) -> Self {
        let head = number / denominator;
        let remainder = number % denominator;

        DivideResult {
            head,
            remainder
        }
    }
}

#[cfg(test)]
mod test_divide_result {
    use super::*;

    #[test]
    fn test_12() {
        let dr = DivideResult::from(12, 10);
        assert_eq!(dr.head, 1);
        assert_eq!(dr.remainder, 2);
    }

    #[test]
    fn test_21() {
        let dr = DivideResult::from(21, 10);
        assert_eq!(dr.head, 2);
        assert_eq!(dr.remainder, 1);
    }

    #[test]
    fn test_123() {
        let dr = DivideResult::from(123, 100);
        assert_eq!(dr.head, 1);
        assert_eq!(dr.remainder, 23);
    }

    #[test]
    fn test_2345() {
        let dr = DivideResult::from(2345, 1000);
        assert_eq!(dr.head, 2);
        assert_eq!(dr.remainder, 345);
    }
}