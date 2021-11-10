use parse_display::{Display, FromStr};
use ulid::Ulid;

use super::error;
use super::money::Deposit;

#[derive(PartialEq, Debug, Builder, Default)]
#[builder(pattern = "owned", setter(into))]
pub struct Wallet {
    id: WalletID,
    deposit: Deposit,
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
            .map_err(|_| error::WalletError::Unexpected)
    }
}

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
