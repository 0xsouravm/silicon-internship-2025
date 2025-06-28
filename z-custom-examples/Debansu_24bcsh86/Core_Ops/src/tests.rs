#[cfg(test)]
mod tests {
    use crate::calculator::*;
    use crate::helper::*; // Unable to use super::helper::*;

    #[test]
    fn test_add() {
        assert_eq!(simple_add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(3));
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Debansu"), "Hello Debansu!");
    }
}