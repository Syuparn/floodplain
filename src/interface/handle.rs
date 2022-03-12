use tonic::{Request, Response, Status, Code};
use super::converter::{Converter, ConverterImpl};
use crate::domain::error::WalletError;
use super::methodtype::{Method, CreateMethod, GetMethod, DeleteMethod};
use super::super::usecase::port::Port;
use super::super::usecase::create::{CreateInputData, CreateOutputData};
use super::super::usecase::get::{GetInputData, GetOutputData};
use super::super::usecase::delete::{DeleteInputData, DeleteOutputData};

pub trait RequestHandler {
    type Method: Method;
    type Converter: Converter<Self::Method>;

    // NOTE: `Self::Method::InputData` raises ambiguous associated type error
    fn exec(&self, input: <<Self as RequestHandler>::Converter as Converter<Self::Method>>::InputData) -> Result<<<Self as RequestHandler>::Converter as Converter<Self::Method>>::OutputData, WalletError>;

    fn handle(&self, req: Request<<<Self as RequestHandler>::Converter as Converter<Self::Method>>::Req>) -> Result<Response<<<Self as RequestHandler>::Converter as Converter<Self::Method>>::Res>, Status> {
        println!(
            "request: {:?} (from {:?})",
            req.get_ref(),
            req.remote_addr()
        );

        let input = Self::Converter::decode(&req.get_ref());
        let output = self.exec(input)
            .map_err(|e| Status::new(Code::Internal, e.to_string()))?;

        let res = Self::Converter::encode(output);

        // TODO: use logging instead
        println!("response: {:?}", res);

        // return response
        Ok(Response::new(res))
    }
}

pub struct RequestHandlerImpl<P, M>
where
    P: Port,
    M: Method,
{
    port: P,
    _method: std::marker::PhantomData<M>,
}

impl<P, M> RequestHandlerImpl<P, M>
where
    P: Port,
    M: Method,
{
    pub fn new(port: P) -> Self {
        RequestHandlerImpl{port: port, _method: std::marker::PhantomData}
    } 
}

// FIXME: remove boilerplates below

impl<P> RequestHandler for RequestHandlerImpl<P, CreateMethod>
where
    P: Port<In = CreateInputData, Out = CreateOutputData>,
{
    type Method = CreateMethod;
    type Converter = ConverterImpl<Self::Method>;

    fn exec(&self, input: CreateInputData) -> Result<CreateOutputData, WalletError> {
        self.port.exec(input)
    }
}

impl<P> RequestHandler for RequestHandlerImpl<P, GetMethod>
where
    P: Port<In = GetInputData, Out = GetOutputData>,
{
    type Method = GetMethod;
    type Converter = ConverterImpl<Self::Method>;

    fn exec(&self, input: GetInputData) -> Result<GetOutputData, WalletError> {
        self.port.exec(input)
    }
}

impl<P> RequestHandler for RequestHandlerImpl<P, DeleteMethod>
where
    P: Port<In = DeleteInputData, Out = DeleteOutputData>,
{
    type Method = DeleteMethod;
    type Converter = ConverterImpl<Self::Method>;

    fn exec(&self, input: DeleteInputData) -> Result<DeleteOutputData, WalletError> {
        self.port.exec(input)
    }
}
