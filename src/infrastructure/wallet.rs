extern crate diesel;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
// necessary for query
use diesel::prelude::*;

use super::super::domain::error;
use super::super::domain::wallet::{Wallet, WalletFactory, WalletID, WalletRepository};
use super::schema::wallet;
use crate::diesel::RunQueryDsl; // NOTE: nessessary to use trait method `execute`
use crate::domain::money::MoneyHolder; // NOTE: nessessary to use method `amount`, `currency

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Insertable)]
#[table_name = "wallet"]
pub struct NewWalletModel<'a> {
    pub id: &'a str,
    pub deposit: &'a i64,
    pub currency: &'a str,
}

#[derive(Queryable, Debug)]
pub struct WalletModel {
    pub id: String,
    pub deposit: i64,
    pub currency: Option<String>,
}

pub struct WalletRepositoryImpl<T: WalletFactory> {
    // NOTE: WalletRepositoryImpl cannot hold pgconnection directly because it is not thread-safe and
    //       cannot be used for async function
    pool: Pool,
    wallet_factory: T,
}

impl<T> WalletRepositoryImpl<T>
where
    T: WalletFactory,
{
    pub fn new(pool: Pool, wallet_factory: T) -> Self {
        WalletRepositoryImpl {
            pool: pool,
            wallet_factory: wallet_factory,
        }
    }
}

impl<T> WalletRepository for WalletRepositoryImpl<T>
where
    T: WalletFactory,
{
    fn save(&self, wallet: &Wallet) -> Result<(), error::WalletError> {
        let new_wallet = NewWalletModel {
            id: &wallet.id.to_string(),
            deposit: &(wallet.amount() as i64),
            currency: &wallet.currency().to_string(),
        };

        let conn = self
            .pool
            .get()
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        diesel::insert_into(wallet::table)
            .values(&new_wallet)
            .execute(&*conn) // NOTE: deref to PgConnection by `*` (https://github.com/sfackler/r2d2/issues/37)
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        Ok(())
    }

    fn get(&self, id: &WalletID) -> Result<Wallet, error::WalletError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        let w = wallet::table
            .select((wallet::id, wallet::deposit, wallet::currency))
            .filter(wallet::id.eq(id.to_string()))
            .first::<WalletModel>(&*conn)
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        let deposit = u64::try_from(w.deposit)
            .map_err(|_| error::WalletError::Unexpected(format!("{} exceeds u64", w.deposit)))?;
        let currency = w.currency.unwrap_or(String::from("JPY"));

        self.wallet_factory.reconstruct(w.id, deposit, currency)
    }

    fn delete(&self, wallet: Wallet) -> Result<(), error::WalletError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        diesel::delete(wallet::table.filter(wallet::id.eq(wallet.id.to_string())))
            .execute(&*conn) // NOTE: deref to PgConnection by `*` (https://github.com/sfackler/r2d2/issues/37)
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::domain::wallet::{
        MockWalletFactory, WalletFactory, WalletFactoryImpl,
    };
    use super::super::client;
    use super::*;

    #[test]
    fn wallet_save_test() {
        // TODO: use test DB
        // setup: delete all records
        let pool = client::connection_pool();
        diesel::delete(wallet::table)
            .execute(&*pool.get().unwrap())
            .unwrap();

        // main test
        let w = WalletFactoryImpl {}
            .reconstruct(String::from("mywallet"), 100, String::from("JPY"))
            .unwrap();
        let r = WalletRepositoryImpl {
            pool: pool,
            wallet_factory: MockWalletFactory::new(),
        };
        assert_eq!(r.save(&w), Ok(()));
    }

    #[test]
    fn wallet_get_test() {
        // TODO: use test DB
        // setup: delete all records and save one record
        let pool = client::connection_pool();
        diesel::delete(wallet::table)
            .execute(&*pool.get().unwrap())
            .unwrap();
        let w = WalletFactoryImpl {}
            .reconstruct(String::from("abc"), 100, String::from("JPY"))
            .unwrap();

        insert_wallet(&w).unwrap();

        // main test
        let r = WalletRepositoryImpl {
            pool: pool,
            wallet_factory: WalletFactoryImpl {},
        };
        assert_eq!(r.get(&w.id), Ok(w));
    }

    #[test]
    fn wallet_delete_test() {
        // TODO: use test DB
        // setup: delete all records and save one record
        let pool = client::connection_pool();
        diesel::delete(wallet::table)
            .execute(&*pool.get().unwrap())
            .unwrap();

        let w1 = WalletFactoryImpl {}
            .reconstruct(String::from("w1"), 100, String::from("JPY"))
            .unwrap();
        let w2 = WalletFactoryImpl {}
            .reconstruct(String::from("w2"), 100, String::from("JPY"))
            .unwrap();

        insert_wallet(&w1).unwrap();
        insert_wallet(&w2).unwrap();

        // main test
        let r = WalletRepositoryImpl {
            pool: pool,
            wallet_factory: WalletFactoryImpl {},
        };
        assert_eq!(r.delete(w1), Ok(()));

        // other wallets should be remained
        let count = wallet::table
            .count()
            .execute(&*client::connection_pool().get().unwrap())
            .unwrap();
        assert_eq!(count, 1);
        assert_eq!(r.get(&w2.id), Ok(w2));
    }

    // test helper
    fn insert_wallet(wallet: &Wallet) -> Result<(), error::WalletError> {
        let w = NewWalletModel {
            id: &wallet.id.to_string(),
            deposit: &(wallet.amount() as i64),
            currency: &wallet.currency().to_string(),
        };

        let pool = client::connection_pool();

        diesel::insert_into(wallet::table)
            .values(&w)
            .execute(&*pool.get().unwrap())
            .map_err(|e| error::WalletError::Unexpected(e.to_string()))?;
        Ok(())
    }
}
