pub fn is_positive(x: i32) -> bool {
    x > 0
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

pub fn divide(x: i32, y: i32) -> i32 {
    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_positive_test() {
        assert!(is_positive(5));
    }

    #[test]
    fn add_test() {
        assert_eq!(add(5, 3), 8);
    }

    #[test]
    fn subtract_test() {
        assert_ne!(subtract(5, 3), 3);
    }

    #[test]
    fn multiply_test() {
        assert_ne!(multiply(5, 3), 10);
    }

    #[test]
    #[ignore]
    fn divide_test() {
        let result = divide(8, 3);
        assert_eq!(result, 3, "{} 需要四舍五入成整数 3", result);
    }
}
