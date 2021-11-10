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

struct CreateInteractor {
    wallet_repository: Box<dyn WalletRepository>,
    wallet_factory: WalletFactory,
}

impl Port<CreateInputData, CreateOutputData> for CreateInteractor {
    fn exec(&self, _: CreateInputData) -> Result<CreateOutputData, WalletError> {
        let wallet = self.wallet_factory.new()?;
        self.wallet_repository.save(&wallet)?;
        Ok(CreateOutputData { wallet: wallet })
    }
}

// TODO: use adorn for logging
