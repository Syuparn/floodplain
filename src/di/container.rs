// TODO: introduce CI container

use super::super::domain::wallet::WalletFactoryImpl;
use super::super::infrastructure::wallet::WalletRepositoryImpl;
use super::super::infrastructure::client::connection_pool;
use super::super::interface::service::WalletServiceImpl;
use super::super::usecase::create::CreateInteractor;

pub fn new_controller() -> WalletServiceImpl<CreateInteractor<WalletRepositoryImpl,WalletFactoryImpl>> {
    let wallet_repository = WalletRepositoryImpl::new(connection_pool());
    let wallet_factory = WalletFactoryImpl{};

    let interactor = CreateInteractor::new(wallet_repository, wallet_factory);

    let controller = WalletServiceImpl::new(interactor);

    controller
}
