extern crate diesel;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use super::super::domain::error;
use super::super::domain::wallet::{Wallet, WalletRepository};
use super::schema::wallet;
use crate::diesel::RunQueryDsl; // NOTE: nessessary to use trait method `execute`
use crate::domain::money::MoneyHolder; // NOTE: nessessary to use method `amount`, `currency

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Insertable)]
#[table_name = "wallet"]
pub struct NewWallet<'a> {
    pub id: &'a str,
    pub deposit: &'a i64,
    pub currency: &'a str,
}

pub struct WalletRepositoryImpl {
    // NOTE: WalletRepositoryImpl cannot hold pgconnection directly because it is not thread-safe and
    //       cannot be used for async function
    pool: Pool,
}

impl WalletRepositoryImpl {
    pub fn new(pool: Pool) -> Self {
        WalletRepositoryImpl {
            pool: pool,
        }
    }
}

impl WalletRepository for WalletRepositoryImpl {
    fn save(&self, wallet: &Wallet) -> Result<(), error::WalletError> {
        let new_wallet = NewWallet {
            id: &wallet.id.to_string(),
            deposit: &(wallet.amount() as i64),
            currency: &wallet.currency().to_string(),
        };

        let conn = self.pool.get()
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        diesel::insert_into(wallet::table)
            .values(&new_wallet)
            .execute(&*conn) // NOTE: deref to PgConnection by `*` (https://github.com/sfackler/r2d2/issues/37)
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::domain::wallet::{WalletFactory, WalletFactoryImpl};
    use super::super::client;
    use super::*;

    #[test]
    fn wallet_save_test() {
        // TODO: use test DB
        // setup: delete all records
        let pool = client::connection_pool();
        diesel::delete(wallet::table).execute(&*pool.get().unwrap()).unwrap();

        // main test
        let w = WalletFactoryImpl {}
            .reconstruct(String::from("abc"), 100, String::from("JPY"))
            .unwrap();
        let r = WalletRepositoryImpl { pool: pool };
        assert_eq!(r.save(&w), Ok(()));
    }
}
