use tonic::{Request, Response, Status, Code};

use walletgrpc::wallet_service_server::{WalletService};
use walletgrpc::{CreateReply, CreateRequest, Wallet};

use super::super::domain::money::MoneyHolder;
use super::super::usecase::port::Port;
use super::super::usecase::create::{CreateInputData, CreateOutputData};

pub mod walletgrpc {
    // import generated gRPC code
    tonic::include_proto!("wallet");
}

#[derive(Default)] // add default() method
pub struct WalletServiceImpl<T>
where
    T: Port<CreateInputData, CreateOutputData>
{
    port: T,
}

impl<T> WalletServiceImpl<T>
where
    T: Port<CreateInputData, CreateOutputData>
{
    pub fn new(port: T) -> Self {
        WalletServiceImpl{
            port: port,
        }
    }

    fn encode(&self, out: CreateOutputData) -> CreateReply {
        let w = out.wallet;

        CreateReply{
            wallet: Some(Wallet{
                id: w.id.to_string(),
                deposit: w.amount(),
                currency: w.currency().to_string(),  
            }),
        }
    }
}

#[tonic::async_trait]
impl<T> WalletService for WalletServiceImpl<T>
where
    T: Port<CreateInputData, CreateOutputData> + Send + Sync + 'static
{
    async fn create(&self, req: Request<CreateRequest>) -> Result<Response<CreateReply>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        let input = CreateInputData{};
        let output = self.port.exec(input)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let reply = self.encode(output);

        // TODO: use logging instead
        println!("response: {:?}", reply);

        // return response
        Ok(Response::new(reply))
    }
}

// TODO: test
