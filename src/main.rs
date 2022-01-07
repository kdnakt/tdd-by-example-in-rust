fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Money {
    amount: i64,
    currency: String,
}

impl Money {
    fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency.to_string()
        }
    }
    fn plus(&self, addend: Money) -> Money {
        Money {
            amount: self.amount + addend.amount,
            currency: self.currency.to_string()
        }
    }
}

fn dollar(amount: i64) -> Money {
    Money {
        amount,
        currency: "USD".to_string(),
    }
}

fn franc(amount: i64) -> Money {
    Money {
        amount,
        currency: "CHF".to_string(),
    }
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
    #[test]
    fn test_currency() {
        assert_eq!("USD", dollar(1).currency);
        assert_eq!("CHF", franc(1).currency);
    }
    #[test]
    fn test_simple_addition() {
        let sum = dollar(5).plus(dollar(5));
        assert_eq!(dollar(10), sum);
    }
}