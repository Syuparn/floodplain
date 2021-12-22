#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod di;
mod domain;
mod usecase;
mod interface;
mod infrastructure;

use tonic::transport::Server;

use di::container::new_controller;
use interface::service::walletgrpc::wallet_service_server::WalletServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let svc = new_controller();

    println!("WalletServer listening on {}", addr);

    Server::builder()
        .add_service(WalletServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
