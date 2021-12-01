use mockall::{automock, predicate::*};
use parse_display::{Display, FromStr};
use ulid::Ulid;

use super::error;
use super::money::{Currency, Deposit, MoneyHolder};

#[derive(PartialEq, Debug, Builder, Default)]
#[builder(pattern = "owned", setter(into))]
pub struct Wallet {
    pub id: WalletID,
    deposit: Deposit,
}

// MoneyHolder is used for ORM etc., otherwise handlers must dig into them!
// TODO: make inner fields private even though their values have to be referred
impl MoneyHolder for Wallet {
    fn currency(self: &Self) -> &Currency {
        &self.deposit.0.currency
    }

    fn amount(self: &Self) -> u64 {
        self.deposit.0.amount
    }
}

#[derive(Display, PartialEq, Debug, FromStr, Default)]
#[display("wallet-{0}")]
pub struct WalletID(String);

pub struct WalletFactory {}

impl WalletFactory {
    pub fn create(&self) -> Result<Wallet, error::WalletError> {
        WalletBuilder::default()
            .id(WalletID(Ulid::new().to_string()))
            .deposit(Deposit::default())
            .build()
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))
    }

    // only for infrastracture and tests!
    pub fn reconstruct(
        &self,
        id: String,
        deposit: u64,
        currency: String,
    ) -> Result<Wallet, error::WalletError> {
        let d = Deposit::new(deposit, currency)?;
        Ok(Wallet {
            id: WalletID(id),
            deposit: d,
        })
    }
}

#[cfg_attr(test, automock)]
pub trait WalletRepository {
    fn save(&self, wallet: &Wallet) -> Result<(), error::WalletError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_id_from_string() {
        assert_eq!(
            "wallet-01FJE5QFC7W7ZS1JN5MR9YVRZW".parse(),
            Ok(WalletID(String::from("01FJE5QFC7W7ZS1JN5MR9YVRZW")))
        );
        assert!("exchange-01FJE5QFC7W7ZS1JN5MR9YVRZW"
            .parse::<WalletID>()
            .is_err());
    }

    #[test]
    fn wallet_builder() {
        assert_eq!(
            WalletBuilder::default()
                .id(WalletID(String::from("01FJE5QFC7W7ZS1JN5MR9YVRZW")))
                .deposit(Deposit::default())
                .build()
                .unwrap(),
            Wallet {
                id: WalletID(String::from("01FJE5QFC7W7ZS1JN5MR9YVRZW")),
                deposit: Deposit::default(),
            }
        );
    }
}
