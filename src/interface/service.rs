use tonic::{Request, Response, Status};

use walletgrpc::wallet_service_server::{WalletService};
use walletgrpc::{CreateReply, CreateRequest, Wallet};

pub mod walletgrpc {
    // import generated gRPC code
    tonic::include_proto!("wallet");
}

#[derive(Default)] // add default() method
pub struct WalletServiceImpl {}

#[tonic::async_trait]
impl WalletService for WalletServiceImpl {
    async fn create(&self, req: Request<CreateRequest>) -> Result<Response<CreateReply>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        // TODO: impl
        let reply = walletgrpc::CreateReply {
            wallet: Some(Wallet{
                id: String::from("abc"),
                deposit: 0,
                currency: String::from("JPY"),  
            }),
        };

        println!("response: {:?}", reply);

        // return response
        Ok(Response::new(reply))
    }
}
