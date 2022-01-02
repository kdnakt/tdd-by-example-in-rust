fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multiplication() {
        let five = Dollar { amount: 5 };
        five.times(2);
        assert_eq!(10, five.amount);
    }
}