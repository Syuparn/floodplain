// TODO: introduce CI container

use super::super::domain::wallet::WalletFactoryImpl;
use super::super::infrastructure::client::connection_pool;
use super::super::infrastructure::wallet::WalletRepositoryImpl;
use super::super::interface::service::WalletServiceImpl;
use super::super::usecase::create::CreateInteractor;
use super::super::usecase::delete::DeleteInteractor;
use super::super::usecase::get::GetInteractor;

pub fn new_controller() -> WalletServiceImpl<
    CreateInteractor<WalletRepositoryImpl<WalletFactoryImpl>, WalletFactoryImpl>,
    GetInteractor<WalletRepositoryImpl<WalletFactoryImpl>>,
    DeleteInteractor<WalletRepositoryImpl<WalletFactoryImpl>>,
> {
    let wallet_repository = WalletRepositoryImpl::new(connection_pool(), WalletFactoryImpl {});
    let wallet_factory = WalletFactoryImpl {};

    let create_interactor = CreateInteractor::new(wallet_repository, wallet_factory);

    // TODO: use repository/factory singletons
    let wallet_repository = WalletRepositoryImpl::new(connection_pool(), WalletFactoryImpl {});
    let get_interactor = GetInteractor::new(wallet_repository);

    let wallet_repository = WalletRepositoryImpl::new(connection_pool(), WalletFactoryImpl {});
    let delete_interactor = DeleteInteractor::new(wallet_repository);

    let controller = WalletServiceImpl::new(create_interactor, get_interactor, delete_interactor);

    controller
}
