use tonic::{Request, Response, Status};

use walletgrpc::wallet_service_server::{WalletService};
use walletgrpc::{CreateRequest, CreateResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse};

use super::handle::{RequestHandler, RequestHandlerImpl};
use super::methodtype::{CreateMethod, GetMethod, DeleteMethod};
use super::super::usecase::port::Port;
use super::super::usecase::create::{CreateInputData, CreateOutputData};
use super::super::usecase::get::{GetInputData, GetOutputData};
use super::super::usecase::delete::{DeleteInputData, DeleteOutputData};

pub mod walletgrpc {
    // import generated gRPC code
    tonic::include_proto!("wallet");
}

pub struct WalletServiceImpl<S, T, U>
where
    S: Port<In = CreateInputData, Out = CreateOutputData>,
    T: Port<In = GetInputData, Out = GetOutputData>,
    U: Port<In = DeleteInputData, Out = DeleteOutputData>,
{
    create_handler: RequestHandlerImpl<S, CreateMethod>,
    get_handler: RequestHandlerImpl<T, GetMethod>,
    delete_handler: RequestHandlerImpl<U, DeleteMethod>,
}

impl<S, T, U> WalletServiceImpl<S, T, U>
where
    S: Port<In = CreateInputData, Out = CreateOutputData>,
    T: Port<In = GetInputData, Out = GetOutputData>,
    U: Port<In = DeleteInputData, Out = DeleteOutputData>,
{
    pub fn new(create_port: S, get_port: T, delete_port: U) -> Self {
        WalletServiceImpl{
            create_handler: RequestHandlerImpl::new(create_port),
            get_handler: RequestHandlerImpl::new(get_port),
            delete_handler: RequestHandlerImpl::new(delete_port),
        }
    }
}

#[tonic::async_trait]
impl<S, T, U> WalletService for WalletServiceImpl<S, T, U>
where
    S: Port<In = CreateInputData, Out = CreateOutputData> + Send + Sync + 'static,
    T: Port<In = GetInputData, Out = GetOutputData> + Send + Sync + 'static,
    U: Port<In = DeleteInputData, Out = DeleteOutputData> + Send + Sync + 'static,
{
    async fn create(&self, req: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        self.create_handler.handle(req)
    }

    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        self.get_handler.handle(req)
    }

    async fn delete(&self, req: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        self.delete_handler.handle(req)
    }
}
