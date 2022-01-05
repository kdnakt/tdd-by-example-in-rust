use crate::{
    Money::*,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Money {
    Dollar {
        amount: i64,
    },
    Franc {
        amount: i64,
    },
}

impl Money {
    fn times(&self, multiplier: i64) -> Money {
        match self {
            Dollar{amount} => Dollar { amount: amount * multiplier },
            Franc{amount} => Franc { amount: amount * multiplier },
        }
    }
}

fn dollar(amount: i64) -> Money {
    Dollar{amount}
}

fn franc(amount: i64) -> Money {
    Franc{amount}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = dollar(5);
        assert_eq!(dollar(10), five.times(2));
        assert_eq!(dollar(15), five.times(3));
    }
    #[test]
    fn test_equality() {
        assert_eq!(dollar(5), dollar(5));
        assert_ne!(dollar(5), dollar(6));
        assert_eq!(franc(5), franc(5));
        assert_ne!(franc(5), franc(6));
        assert_ne!(franc(5), dollar(5));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = franc(5);
        assert_eq!(franc(10), five.times(2));
        assert_eq!(franc(15), five.times(3));
    }
}