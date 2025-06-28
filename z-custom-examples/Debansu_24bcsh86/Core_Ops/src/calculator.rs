use super::helper::*; // use crate::helper::*;

pub fn simple_add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn add_with_greeting(a: i32, b: i32, name: &str) -> String {
    let sum = simple_add(a, b);
    let greeting = greet(name); // crate::helper::greet(name);
    format!("{greeting} The Sum Of {a} & {b}: {sum}.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(simple_add(2, 3), 5);
        // assert_eq!(simple_add(2, 3), 4, "Result Must Be: 5");
    }

    #[test]
    fn test_multiply(){
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn test_is_even(){
        assert!(is_even(4));
        assert!(!is_even(3));
        assert_ne!(is_even(5), true)
    }
}