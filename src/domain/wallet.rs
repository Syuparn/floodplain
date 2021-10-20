use parse_display::{Display, FromStr};

use super::money::{Deposit};

#[derive(PartialEq, Debug, Default)]
pub struct Wallet {
    id: WalletID,
    deposit: Deposit,
}

#[derive(Display, PartialEq, Debug, FromStr, Default)]
#[display("wallet-{0}")]
struct WalletID(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_id_from_string() {
        assert_eq!("wallet-01FJE5QFC7W7ZS1JN5MR9YVRZW".parse(), Ok(WalletID(String::from("01FJE5QFC7W7ZS1JN5MR9YVRZW"))));
        assert!("exchange-01FJE5QFC7W7ZS1JN5MR9YVRZW".parse::<WalletID>().is_err());
    }
}
