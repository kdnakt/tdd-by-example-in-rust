// Std
use std::{
    any::Any,
    collections::HashMap,
};

fn main() {
    println!("Hello, world!");
}

trait Expression {
    fn as_any(&self) -> &dyn Any;
    fn reduce(&self, bank: &Bank, to: String) -> Money;
    fn plus(&self, addend: Box<dyn Expression>) -> Box<dyn Expression>;
}

#[derive(Debug, PartialEq)]
struct Money {
    amount: i64,
    currency: String,
}

impl Expression for Money {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn reduce(&self, bank: &Bank, to: String) -> Money {
        let rate = bank.rate(self.currency.to_string(), to.to_string());
        Money {
            amount: self.amount / rate,
            currency: to,
        }
    }
    fn plus(&self, addend: Box<dyn Expression>) -> Box<dyn Expression> {
        Box::new(Sum {
            augend: Box::new(Money{
                amount: self.amount,
                currency: self.currency.to_string(),
            }),
            addend,
        })
    }
}

impl Money {
    fn times(&self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
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

struct Sum {
    augend: Box<dyn Expression>,
    addend: Box<dyn Expression>,
}

impl Expression for Sum {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn reduce(&self, bank: &Bank, to: String) -> Money {
        Money {
            amount: self.augend.reduce(bank, to.to_string()).amount
                    + self.addend.reduce(bank, to.to_string()).amount,
            currency: to,
        }
    }
    fn plus(&self, addened: Box<dyn Expression>) -> Box<dyn Expression> {
        // TODO: fix
        Box::new(dollar(1))
    }
}

struct Bank {
    rates: HashMap<Pair, i64>
}

impl Bank {
    fn new() -> Bank {
        let rates = HashMap::new();
        Bank { rates }
    }
    fn reduce(&self, source: Box<dyn Expression>, to: String) -> Money {
        source.reduce(self, to)
    }
    fn add_rate(&mut self, from: String, to: String, rate: i64) {
        self.rates.insert(Pair { from, to }, rate);
    }
    fn rate(&self, from: String, to: String) -> i64 {
        if from == to {
            1
        } else {
            self.rates.get(&Pair { from, to }).unwrap().clone()
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Pair {
    from: String,
    to: String,
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
        let five = dollar(5);
        let sum = five.plus(Box::new(dollar(5)));
        let bank = Bank::new();
        let reduced = bank.reduce(sum, "USD".to_string());
        assert_eq!(dollar(10), reduced);
    }
    #[test]
    fn test_plus_returns_sum() {
        let five = dollar(5);
        let result = five.plus(Box::new(dollar(5)));
        let sum = result.as_any()
                .downcast_ref::<Sum>()
                .expect("Wasn't a Sum");
        let augend = sum.augend.as_any()
                .downcast_ref::<Money>()
                .expect("Wasn't a Money");
        assert_eq!(&five, augend);
        let addend = sum.addend.as_any()
                .downcast_ref::<Money>()
                .expect("Wasn't a Money");
        assert_eq!(&five, addend);
    }
    #[test]
    fn test_reduce_sum() {
        let sum = Box::new(Sum {
            augend: Box::new(dollar(3)),
            addend: Box::new(dollar(4)),
        });
        let bank = Bank::new();
        let result = bank.reduce(sum, "USD".to_string());
        assert_eq!(dollar(7), result);
    }
    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(Box::new(dollar(1)), "USD".to_string());
        assert_eq!(dollar(1), result);
    }
    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate("CHF".to_string(), "USD".to_string(), 2);
        let result = bank.reduce(Box::new(franc(2)), "USD".to_string());
        assert_eq!(dollar(1), result);
    }
    #[test]
    fn test_identity_rate() {
        let bank = Bank::new();
        assert_eq!(1, bank.rate("USD".to_string(), "USD".to_string()));
    }
    #[test]
    fn test_mixed_addition() {
        let five_bucks = Box::new(dollar(5));
        let ten_francs = Box::new(franc(10));
        let mut bank = Bank::new();
        bank.add_rate("CHF".to_string(), "USD".to_string(), 2);
        let result = bank.reduce(five_bucks.plus(ten_francs), "USD".to_string());
        assert_eq!(dollar(10), result);
    }
}