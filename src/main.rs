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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar { amount: 5 };
        assert_eq!(Dollar { amount: 10}, five.times(2));
        assert_eq!(Dollar { amount: 15}, five.times(3));
    }
    #[test]
    fn test_equality() {
        assert_eq!(Dollar { amount: 5}, Dollar { amount: 5});
        assert_ne!(Dollar { amount: 5}, Dollar { amount: 6});
        assert_eq!(Franc { amount: 5}, Franc { amount: 5});
        assert_ne!(Franc { amount: 5}, Franc { amount: 6});
        assert_ne!(Franc { amount: 5}, Dollar { amount: 5});
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc { amount: 5 };
        assert_eq!(Franc { amount: 10}, five.times(2));
        assert_eq!(Franc { amount: 15}, five.times(3));
    }
}