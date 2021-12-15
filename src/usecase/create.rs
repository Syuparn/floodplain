use super::port::Port;
use crate::domain::error::WalletError;
use crate::domain::wallet::{Wallet, WalletFactory, WalletRepository};

#[derive(PartialEq, Debug)]
struct CreateInputData {
    // TODO: add data
}

#[derive(PartialEq, Debug)]
struct CreateOutputData {
    wallet: Wallet,
}

struct CreateInteractor<S: WalletRepository, T: WalletFactory> {
    wallet_repository: S,
    wallet_factory: T,
}

impl<S, T> Port<CreateInputData, CreateOutputData> for CreateInteractor<S, T>
where
    S: WalletRepository,
    T: WalletFactory,
{
    fn exec(&self, _: CreateInputData) -> Result<CreateOutputData, WalletError> {
        let wallet = self.wallet_factory.create()?;
        self.wallet_repository.save(&wallet)?;
        Ok(CreateOutputData { wallet })
    }
}

// TODO: use adorn for logging

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::wallet::{Wallet, MockWalletRepository, MockWalletFactory, WalletFactoryImpl};

    // helper function to create dummy wallet
    // NOTE: since Wallet is not a reference and can be used only once, call this each time where wallet is required
    fn wallet() -> Wallet {
        WalletFactoryImpl{}.reconstruct(String::from("abc"), 0, String::from("JPY")).unwrap()
    }

    #[test]
    fn create() {
        let mut wallet_repository = MockWalletRepository::new();
        // mock save() method
        wallet_repository.expect_save().returning(|_| Ok(()));

        let mut wallet_factory = MockWalletFactory::new();
        // mock create() method
        wallet_factory.expect_create().returning(|| Ok(wallet()));

        let interactor = CreateInteractor{
            wallet_repository: wallet_repository,
            wallet_factory: wallet_factory,
        };

        let actual = interactor.exec(CreateInputData{});

        assert_eq!(actual, Ok(CreateOutputData{wallet: wallet()}));
    }
}
