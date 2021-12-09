#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod domain;
mod usecase;
mod interface;
mod infrastructure;

use tonic::transport::Server;

use interface::service::walletgrpc::wallet_service_server::{WalletServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let svc = interface::service::WalletServiceImpl::default();

    println!("WalletServer listening on {}", addr);

    Server::builder()
        .add_service(WalletServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
