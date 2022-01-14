// Std
use std::{
    any::Any,
    collections::HashMap,
};

// Internal
use crate::{
    Currency::*,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum Currency {
    USD,
    CHF,
}

trait Expression {
    fn as_any(&self) -> &dyn Any;
    fn reduce(&self, bank: &Bank, to: Currency) -> Money;
    fn plus(self, addend: Box<dyn Expression>) -> Box<dyn Expression>;
}

#[derive(Debug, PartialEq)]
struct Money {
    amount: i64,
    currency: Currency,
}

impl Expression for Money {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        let rate = bank.rate(self.currency, to);
        Money {
            amount: self.amount / rate,
            currency: to,
        }
    }
    fn plus(self, addend: Box<dyn Expression>) -> Box<dyn Expression> {
        Box::new(Sum {
            augend: Box::new(self),
            addend,
        })
    }
}

impl Money {
    fn dollar(amount: i64) -> Money {
        Money {
            amount,
            currency: USD,
        }
    }
    fn franc(amount: i64) -> Money {
        Money {
            amount,
            currency: CHF,
        }
    }
    fn times(self, multiplier: i64) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency,
        }
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
    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        Money {
            amount: self.augend.reduce(bank, to).amount
                    + self.addend.reduce(bank, to).amount,
            currency: to,
        }
    }
    fn plus(self, addened: Box<dyn Expression>) -> Box<dyn Expression> {
        Box::new(Sum {
            augend: Box::new(self),
            addend: addened,
        })
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
    fn reduce(&self, source: Box<dyn Expression>, to: Currency) -> Money {
        source.reduce(self, to)
    }
    fn add_rate(&mut self, from: Currency, to: Currency, rate: i64) {
        self.rates.insert(Pair { from, to }, rate);
    }
    fn rate(&self, from: Currency, to: Currency) -> i64 {
        if from == to {
            1
        } else {
            self.rates.get(&Pair { from, to }).unwrap().clone()
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Pair {
    from: Currency,
    to: Currency,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = || Money::dollar(5);
        assert_eq!(Money::dollar(10), five().times(2));
        assert_eq!(Money::dollar(15), five().times(3));
    }
    #[test]
    fn test_equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_eq!(Money::franc(5), Money::franc(5));
        assert_ne!(Money::franc(5), Money::franc(6));
        assert_ne!(Money::franc(5), Money::dollar(5));
    }
    #[test]
    fn test_franc_multiplication() {
        let five = || Money::franc(5);
        assert_eq!(Money::franc(10), five().times(2));
        assert_eq!(Money::franc(15), five().times(3));
    }
    #[test]
    fn test_currency() {
        assert_eq!(USD, Money::dollar(1).currency);
        assert_eq!(CHF, Money::franc(1).currency);
    }
    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five.plus(Box::new(Money::dollar(5)));
        let bank = Bank::new();
        let reduced = bank.reduce(sum, USD);
        assert_eq!(Money::dollar(10), reduced);
    }
    #[test]
    fn test_plus_returns_sum() {
        let five = Money::dollar(5);
        let result = five.plus(Box::new(Money::dollar(5)));
        let sum = result.as_any()
                .downcast_ref::<Sum>()
                .expect("Wasn't a Sum");
        let augend = sum.augend.as_any()
                .downcast_ref::<Money>()
                .expect("Wasn't a Money");
        let five = Money::dollar(5);
        assert_eq!(&five, augend);
        let addend = sum.addend.as_any()
                .downcast_ref::<Money>()
                .expect("Wasn't a Money");
        assert_eq!(&five, addend);
    }
    #[test]
    fn test_reduce_sum() {
        let sum = Box::new(Sum {
            augend: Box::new(Money::dollar(3)),
            addend: Box::new(Money::dollar(4)),
        });
        let bank = Bank::new();
        let result = bank.reduce(sum, USD);
        assert_eq!(Money::dollar(7), result);
    }
    #[test]
    fn test_reduce_money() {
        let bank = Bank::new();
        let result = bank.reduce(Box::new(Money::dollar(1)), USD);
        assert_eq!(Money::dollar(1), result);
    }
    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(CHF, USD, 2);
        let result = bank.reduce(Box::new(Money::franc(2)), USD);
        assert_eq!(Money::dollar(1), result);
    }
    #[test]
    fn test_identity_rate() {
        let bank = Bank::new();
        assert_eq!(1, bank.rate(USD, USD));
    }
    #[test]
    fn test_mixed_addition() {
        let five_bucks = Box::new(Money::dollar(5));
        let ten_francs = Box::new(Money::franc(10));
        let mut bank = Bank::new();
        bank.add_rate(CHF, USD, 2);
        let result = bank.reduce(five_bucks.plus(ten_francs), USD);
        assert_eq!(Money::dollar(10), result);
    }
    #[test]
    fn test_sum_plus_money() {
        let five_bucks = Box::new(Money::dollar(5));
        let ten_francs = Box::new(Money::franc(10));
        let mut bank = Bank::new();
        bank.add_rate(CHF, USD, 2);
        let sum = Sum {
            augend: five_bucks,
            addend: ten_francs,
        }.plus(Box::new(Money::dollar(5)));
        let result = bank.reduce(sum, USD);
        assert_eq!(Money::dollar(15), result);
    }
}