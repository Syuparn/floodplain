use parse_display::{Display, FromStr};

use super::error;

#[derive(Display, PartialEq, Debug)]
#[display("{0}")]
pub struct Deposit(Money);

impl Deposit {
    fn new(amount: u64, currency: String) -> Result<Deposit, error::WalletError> {
        let c = currency.parse().map_err(|_| error::WalletError::InvalidCurrency(currency))?;
        Ok(Deposit(Money{
            amount: amount,
            currency: c,
        }))
    }
}

#[derive(Display, PartialEq, Debug)]
#[display("{amount}{currency}")]
struct Money {
    currency: Currency,
    amount: u64,
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Currency {
    JPY,
    USD,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::error;

    #[test]
    fn currency_from_string() {
        assert_eq!("JPY".parse(), Ok(Currency::JPY));
        assert_eq!("USD".parse(), Ok(Currency::USD));
    }

    #[test]
    fn display_money() {
        assert_eq!(Money{currency: Currency::JPY, amount: 100}.to_string(), "100JPY");
    }

    #[test]
    fn display_deposit() {
        assert_eq!(Deposit(Money{currency: Currency::JPY, amount: 100}).to_string(), "100JPY");
    }

    #[test]
    fn new_deposit() {
        // TODO: fix: binary operation `==` cannot be applied to type `std::result::Result<domain::money::Deposit, domain::error::WalletError>`
        assert_eq!(Deposit::new(100, String::from("JPY")).unwrap(), Deposit(Money{currency: Currency::JPY, amount: 100}));
        assert_eq!(Deposit::new(100, String::from("FOO")).unwrap_err(), error::WalletError::InvalidCurrency(String::from("FOO")));
    }
}
