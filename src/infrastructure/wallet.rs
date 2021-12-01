use diesel::pg::PgConnection;

use super::super::domain::error;
use super::super::domain::wallet::{Wallet, WalletRepository};
use super::schema::wallet;
use crate::diesel::RunQueryDsl; // NOTE: nessessary to use trait method `execute`
use crate::domain::money::MoneyHolder; // NOTE: nessessary to use method `amount`, `currency

#[derive(Insertable)]
#[table_name = "wallet"]
pub struct NewWallet<'a> {
    pub id: &'a str,
    pub deposit: &'a i64,
    pub currency: &'a str,
}

pub struct WalletRepositoryImpl<'a> {
    pub conn: &'a PgConnection,
}

impl WalletRepository for WalletRepositoryImpl<'_> {
    fn save(&self, wallet: &Wallet) -> Result<(), error::WalletError> {
        let new_wallet = NewWallet {
            id: &wallet.id.to_string(),
            deposit: &(wallet.amount() as i64),
            currency: &wallet.currency().to_string(),
        };

        diesel::insert_into(wallet::table)
            .values(&new_wallet)
            .execute(self.conn)
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::domain::wallet::WalletFactory;
    use super::super::client;
    use super::*;

    #[test]
    fn wallet_save_test() {
        // TODO: use test DB
        // setup: delete all records
        let conn = client::establish_connection();
        diesel::delete(wallet::table).execute(&conn).unwrap();

        // main test
        let w = WalletFactory {}
            .reconstruct(String::from("abc"), 100, String::from("JPY"))
            .unwrap();
        let r = WalletRepositoryImpl { conn: &conn };
        assert_eq!(r.save(&w), Ok(()));
    }
}
