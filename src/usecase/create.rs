use super::port::Port;
use crate::domain::error::WalletError;
use crate::domain::wallet::{Wallet, WalletFactory, WalletRepository};

#[derive(Debug)]
struct CreateInputData {
    // TODO: add data
}

#[derive(Debug)]
struct CreateOutputData {
    wallet: Wallet,
}

struct CreateInteractor<T: WalletRepository> {
    wallet_repository: T,
    wallet_factory: WalletFactory,
}

impl<T> Port<CreateInputData, CreateOutputData> for CreateInteractor<T>
where
    T: WalletRepository,
{
    fn exec(&self, _: CreateInputData) -> Result<CreateOutputData, WalletError> {
        let wallet = self.wallet_factory.create()?;
        self.wallet_repository.save(&wallet)?;
        Ok(CreateOutputData { wallet })
    }
}

// TODO: use adorn for logging
