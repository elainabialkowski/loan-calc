use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use thiserror::Error;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Currency(f32);

#[derive(Error, Debug)]
pub enum CurrencyError {
    #[error("Could not parse the following value as currency: {0}")]
    ParseError(String),
}

impl Currency {
    pub fn new(value: f32) -> Currency {
        Currency(value)
    }

    fn round(&self) -> Currency {
        let value = self.0;
        format!("{value:.2}").parse::<f32>().map(Currency).unwrap()
    }

    pub fn percent_of(&self, percentage: f32) -> Currency {
        let value = self.0 * (percentage / 100.0);
        Currency(value).round()
    }

    pub fn interest(&self, Interest(rate, compounded): Interest) -> Currency {
        match compounded {
            Compounded::Monthly => self.percent_of(rate).distribute(12),
        }
    }

    pub fn distribute(&self, count: usize) -> Currency {
        Currency(self.0 / count as f32).round()
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

impl FromStr for Currency {
    type Err = CurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.replace('$', "")
            .parse::<f32>()
            .map(Currency)
            .map(|cur| cur.round())
            .map_err(|_| CurrencyError::ParseError(s.to_string()))
    }
}

impl Add for Currency {
    type Output = Currency;

    fn add(self, rhs: Self) -> Self::Output {
        Currency(self.0 + rhs.0).round()
    }
}

impl Sub for Currency {
    type Output = Currency;

    fn sub(self, rhs: Self) -> Self::Output {
        Currency(self.0 - rhs.0).round()
    }
}

impl Div for Currency {
    type Output = Currency;

    fn div(self, rhs: Self) -> Self::Output {
        Currency(self.0 / rhs.0).round()
    }
}

impl Mul for Currency {
    type Output = Currency;

    fn mul(self, rhs: Self) -> Self::Output {
        Currency(self.0 * rhs.0).round()
    }
}

pub enum Compounded {
    Monthly,
}

pub struct Interest(f32, Compounded);

impl Interest {
    pub fn new(rate: f32, compounded: Compounded) -> Interest {
        Interest(rate, compounded)
    }
}
