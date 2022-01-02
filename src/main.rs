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
        assert_eq!(Dollar { amount: 10}, five.times(2));
        assert_eq!(Dollar { amount: 15}, five.times(3));
    }
    #[test]
    fn test_equality() {
        assert_eq!(Dollar { amount: 5}, Dollar { amount: 5});
        assert_ne!(Dollar { amount: 5}, Dollar { amount: 6});
    }
}