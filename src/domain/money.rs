use parse_display::{Display, FromStr};

use super::error;

#[derive(Display, PartialEq, Debug, Default)]
#[display("{0}")]
pub struct Deposit(pub Money);

impl Deposit {
    pub fn new(amount: u64, currency: String) -> Result<Deposit, error::WalletError> {
        let m = Money::new(amount, currency)?;
        Ok(Deposit(m))
    }
}

#[derive(Display, PartialEq, Debug, Default)]
#[display("{amount}{currency}")]
pub struct Money {
    pub currency: Currency,
    pub amount: u64,
}

impl Money {
    fn new(amount: u64, currency: String) -> Result<Money, error::WalletError> {
        let c = currency
            .parse()
            .map_err(|_| error::WalletError::InvalidCurrency(currency))?;
        Ok(Money {
            amount,
            currency: c,
        })
    }
}

pub trait MoneyHolder {
    fn currency(&self) -> &Currency;
    fn amount(&self) -> u64;
}

#[derive(Display, FromStr, PartialEq, Debug)]
pub enum Currency {
    #[display(style = "UPPERCASE")]
    Jpy,
    #[display(style = "UPPERCASE")]
    Usd,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::Jpy
    }
}

#[cfg(test)]
mod tests {
    use super::error;
    use super::*;

    #[test]
    fn currency_from_string() {
        assert_eq!("JPY".parse(), Ok(Currency::Jpy));
        assert_eq!("USD".parse(), Ok(Currency::Usd));
    }

    #[test]
    fn display_money() {
        assert_eq!(
            Money {
                currency: Currency::Jpy,
                amount: 100
            }
            .to_string(),
            "100Jpy"
        );
    }

    #[test]
    fn display_deposit() {
        assert_eq!(
            Deposit(Money {
                currency: Currency::Jpy,
                amount: 100
            })
            .to_string(),
            "100Jpy"
        );
    }

    #[test]
    fn money_new() {
        // TODO: fix: binary operation `==` cannot be applied to type `std::result::Result<domain::money::Deposit, domain::error::WalletError>`
        assert_eq!(
            Money::new(100, String::from("Jpy")).unwrap(),
            Money {
                currency: Currency::Jpy,
                amount: 100
            }
        );
        assert_eq!(
            Money::new(100, String::from("FOO")).unwrap_err(),
            error::WalletError::InvalidCurrency(String::from("FOO"))
        );
    }

    #[test]
    fn deposit_new() {
        // TODO: fix: binary operation `==` cannot be applied to type `std::result::Result<domain::money::Deposit, domain::error::WalletError>`
        assert_eq!(
            Deposit::new(100, String::from("Jpy")).unwrap(),
            Deposit(Money {
                currency: Currency::Jpy,
                amount: 100
            })
        );
        assert_eq!(
            Deposit::new(100, String::from("FOO")).unwrap_err(),
            error::WalletError::InvalidCurrency(String::from("FOO"))
        );
    }

    #[test]
    fn deposit_default() {
        assert_eq!(
            Deposit::default(),
            Deposit(Money {
                amount: 0,
                currency: Currency::Jpy
            })
        );
    }
}
