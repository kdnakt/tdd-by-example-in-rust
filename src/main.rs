fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Dollar {
    amount: i64,
}

impl Dollar {
    fn times(&mut self, multiplier: i64) -> Dollar {
        Dollar { amount: self.amount * multiplier }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let mut five = Dollar { amount: 5 };
        let product = five.times(2);
        assert_eq!(10, product.amount);
        let product = five.times(3);
        assert_eq!(15, product.amount);
    }
    #[test]
    fn test_equality() {
        assert_eq!(Dollar { amount: 5}, Dollar { amount: 5});
        assert_ne!(Dollar { amount: 5}, Dollar { amount: 6});
    }
}