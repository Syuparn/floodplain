use super::port::Port;
use crate::domain::error::WalletError;
use crate::domain::wallet::{WalletID, Wallet, WalletRepository};

#[derive(PartialEq, Debug)]
pub struct GetInputData {
    pub id: String
}

#[derive(PartialEq, Debug)]
pub struct GetOutputData {
    pub wallet: Wallet,
}

pub struct GetInteractor<S>
where
    S: WalletRepository,
{
    wallet_repository: S,
}

impl<S> GetInteractor<S>
where
    S: WalletRepository,
{
    pub fn new(repository: S) -> Self {
        GetInteractor {
            wallet_repository: repository,
        }
    }
}

impl<S> Port for GetInteractor<S>
where
    S: WalletRepository,
{
    type In = GetInputData;
    type Out = GetOutputData;

    fn exec(&self, input: GetInputData) -> Result<GetOutputData, WalletError> {
        let wallet = self.wallet_repository.get(&WalletID(input.id))?;
        Ok(GetOutputData { wallet })
    }
}

// TODO: use adorn for logging

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::wallet::{Wallet, MockWalletRepository, WalletFactory, WalletFactoryImpl};

    // helper function to Get dummy wallet
    // NOTE: since Wallet is not a reference and can be used only once, call this each time where wallet is required
    fn wallet() -> Wallet {
        WalletFactoryImpl{}.reconstruct(String::from("abc"), 0, String::from("JPY")).unwrap()
    }

    #[test]
    fn get() {
        let mut wallet_repository = MockWalletRepository::new();
        // mock get() method
        wallet_repository.expect_get().returning(|_| Ok(wallet()));

        let interactor = GetInteractor{
            wallet_repository: wallet_repository,
        };

        let id = "01FJE5QFC7W7ZS1JN5MR9YVRZW".parse().unwrap();
        let actual = interactor.exec(GetInputData{id: id});

        assert_eq!(actual, Ok(GetOutputData{wallet: wallet()}));
    }
}
