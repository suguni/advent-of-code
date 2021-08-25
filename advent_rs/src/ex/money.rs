use std::str::FromStr;
use std::error::Error;
use std::fmt::{Display, Formatter, Debug};
use std::num::ParseFloatError;

/*
 FROM https://www.youtube.com/watch?v=P2mooqNMxMs
 */

#[derive(Debug, Eq, PartialEq)]
enum Currency {
    Dollar,
    Euro,
    Won,
}

impl FromStr for Currency {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$" => Ok(Currency::Dollar),
            "euro" => Ok(Currency::Euro),
            "won" => Ok(Currency::Won),
            _ => Err(MoneyError::ParseError)
        }
    }
}

#[derive(Debug)]
struct Money {
    amount: f32,
    currency: Currency,
}

impl Money {
    fn new(amount: f32, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency && (self.amount - other.amount).abs() < 0.0001
    }
}

enum MoneyError {
    ParseError,
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoneyError::ParseError => f.write_str("Invalid Money")
        }
    }
}

impl Debug for MoneyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoneyError::ParseError => f.write_str("ParseError")
        }
    }
}

impl Error for MoneyError {
    fn description(&self) -> &str {
        match self {
            MoneyError::ParseError => "ParseError"
        }
    }
}

impl From<ParseFloatError> for MoneyError {
    fn from(_: ParseFloatError) -> Self {
        MoneyError::ParseError
    }
}

impl FromStr for Money {

    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("100 $".parse::<Money>().unwrap(),
                   Money::new(100.0, Currency::Dollar));
    }
}