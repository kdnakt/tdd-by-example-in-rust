fn main() {
    println!("Hello, world!");
}

struct Dollar {
    amount: i64,
}

impl Dollar {
    fn times(&mut self, multiplier: i64) {
        self.amount *= multiplier;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let mut five = Dollar { amount: 5 };
        five.times(2);
        assert_eq!(10, five.amount);
    }
}