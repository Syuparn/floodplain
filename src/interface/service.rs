use tonic::{Request, Response, Status, Code};

use walletgrpc::wallet_service_server::{WalletService};
use walletgrpc::{CreateRequest, CreateResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse, Wallet};

use super::super::domain::money::MoneyHolder;
use super::super::usecase::port::Port;
use super::super::usecase::create::{CreateInputData, CreateOutputData};
use super::super::usecase::get::{GetInputData, GetOutputData};
use super::super::usecase::delete::{DeleteInputData, DeleteOutputData};

pub mod walletgrpc {
    // import generated gRPC code
    tonic::include_proto!("wallet");
}

// FIXME: replace Port with associative type
pub struct WalletServiceImpl<S, T, U>
where
    S: Port<CreateInputData, CreateOutputData>,
    T: Port<GetInputData, GetOutputData>,
    U: Port<DeleteInputData, DeleteOutputData>,
{
    create_port: S,
    get_port: T,
    delete_port: U,
}

impl<S, T, U> WalletServiceImpl<S, T, U>
where
    S: Port<CreateInputData, CreateOutputData>,
    T: Port<GetInputData, GetOutputData>,
    U: Port<DeleteInputData, DeleteOutputData>,
{
    pub fn new(create_port: S, get_port: T, delete_port: U) -> Self {
        WalletServiceImpl{
            create_port: create_port,
            get_port: get_port,
            delete_port: delete_port,
        }
    }

    fn encode_output_create(&self, out: CreateOutputData) -> CreateResponse {
        let w = out.wallet;

        CreateResponse{
            wallet: Some(Wallet{
                id: w.id.to_string(),
                deposit: w.amount(),
                currency: w.currency().to_string(),
            }),
        }
    }

    fn encode_input_get(&self, req: &GetRequest) -> Result<GetInputData, Box<dyn std::error::Error>> {
        Ok(GetInputData{
            id: req.id.clone(),
        })
    }

    fn encode_output_get(&self, out: GetOutputData) -> GetResponse {
        let w = out.wallet;

        GetResponse{
            wallet: Some(Wallet{
                id: w.id.to_string(),
                deposit: w.amount(),
                currency: w.currency().to_string(),
            }),
        }
    }

    fn encode_input_delete(&self, req: &DeleteRequest) -> Result<DeleteInputData, Box<dyn std::error::Error>> {
        Ok(DeleteInputData{
            id: req.id.clone(),
        })
    }

    fn encode_output_delete(&self, _out: DeleteOutputData) -> DeleteResponse {
        DeleteResponse{}
    }
}

#[tonic::async_trait]
impl<S, T, U> WalletService for WalletServiceImpl<S, T, U>
where
    S: Port<CreateInputData, CreateOutputData> + Send + Sync + 'static,
    T: Port<GetInputData, GetOutputData> + Send + Sync + 'static,
    U: Port<DeleteInputData, DeleteOutputData> + Send + Sync + 'static,
{
    async fn create(&self, req: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        let input = CreateInputData{};
        let output = self.create_port.exec(input)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let res = self.encode_output_create(output);

        // TODO: use logging instead
        println!("response: {:?}", res);

        // return response
        Ok(Response::new(res))
    }

    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        let input = self.encode_input_get(req.get_ref())
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;
        let output = self.get_port.exec(input)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let res = self.encode_output_get(output);

        // TODO: use logging instead
        println!("response: {:?}", res);

        // return response
        Ok(Response::new(res))
    }

    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        let input = self.encode_input_delete(req.get_ref())
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;
        let output = self.delete_port.exec(input)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let res = self.encode_output_delete(output);

        // TODO: use logging instead
        println!("response: {:?}", res);

        // return response
        Ok(Response::new(res))
    }
}
