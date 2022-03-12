use super::port::Port;
use crate::domain::error::WalletError;
use crate::domain::wallet::{WalletID, WalletRepository};

#[derive(PartialEq, Debug)]
pub struct DeleteInputData {
    pub id: String
}

#[derive(PartialEq, Debug)]
pub struct DeleteOutputData {}

pub struct DeleteInteractor<S>
where
    S: WalletRepository,
{
    wallet_repository: S,
}

impl<S> DeleteInteractor<S>
where
    S: WalletRepository,
{
    pub fn new(repository: S) -> Self {
        DeleteInteractor {
            wallet_repository: repository,
        }
    }
}

impl<S> Port for DeleteInteractor<S>
where
    S: WalletRepository,
{
    type In = DeleteInputData;
    type Out = DeleteOutputData;

    fn exec(&self, input: DeleteInputData) -> Result<DeleteOutputData, WalletError> {
        let wallet = self.wallet_repository.get(&WalletID(input.id))?;
        self.wallet_repository.delete(wallet)?;
        Ok(DeleteOutputData {})
    }
}

// TODO: use adorn for logging

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::wallet::{Wallet, MockWalletRepository, WalletFactory, WalletFactoryImpl};

    // helper function to Delete dummy wallet
    // NOTE: since Wallet is not a reference and can be used only once, call this each time where wallet is required
    fn wallet() -> Wallet {
        WalletFactoryImpl{}.reconstruct(String::from("01FJE5QFC7W7ZS1JN5MR9YVRZW"), 0, String::from("JPY")).unwrap()
    }

    #[test]
    fn delete() {
        let mut wallet_repository = MockWalletRepository::new();
        // mock methods
        wallet_repository.expect_get().returning(|_| Ok(wallet()));
        wallet_repository.expect_delete().returning(|_| Ok(()));

        let interactor = DeleteInteractor{
            wallet_repository: wallet_repository,
        };

        let id = "01FJE5QFC7W7ZS1JN5MR9YVRZW".parse().unwrap();
        let actual = interactor.exec(DeleteInputData{id: id});

        assert_eq!(actual, Ok(DeleteOutputData{}));
    }
}
