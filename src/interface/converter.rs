use super::service::walletgrpc::{CreateRequest, CreateResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse, Wallet};
use super::super::domain::money::MoneyHolder;
use super::super::usecase::create::{CreateInputData, CreateOutputData};
use super::super::usecase::get::{GetInputData, GetOutputData};
use super::super::usecase::delete::{DeleteInputData, DeleteOutputData};
use super::methodtype::{Method, CreateMethod, GetMethod, DeleteMethod};

pub trait Converter<M: Method> {
    type Req: std::fmt::Debug;
    type Res: std::fmt::Debug;
    type InputData;
    type OutputData;

    fn decode(req: &Self::Req) -> Self::InputData;
    fn encode(out: Self::OutputData) -> Self::Res;
}

pub struct ConverterImpl<M: Method> {
    _phantom: std::marker::PhantomData<M>,
}

impl Converter<CreateMethod> for ConverterImpl<CreateMethod> {
    type Req = CreateRequest;
    type Res = CreateResponse;
    type InputData = CreateInputData;
    type OutputData = CreateOutputData;

    fn decode(_: &CreateRequest) -> CreateInputData {
        CreateInputData{}
    }

    fn encode(out: CreateOutputData) -> CreateResponse {
        let w = out.wallet;

        CreateResponse{
            wallet: Some(Wallet{
                id: w.id.to_string(),
                deposit: w.amount(),
                currency: w.currency().to_string(),
            }),
        }
    }
}

impl Converter<GetMethod> for ConverterImpl<GetMethod> {
    type Req = GetRequest;
    type Res = GetResponse;
    type InputData = GetInputData;
    type OutputData = GetOutputData;

    fn decode(req: &GetRequest) -> GetInputData {
        GetInputData{
            id: req.id.clone(),
        }
    }

    fn encode(out: GetOutputData) -> GetResponse {
        let w = out.wallet;

        GetResponse{
            wallet: Some(Wallet{
                id: w.id.to_string(),
                deposit: w.amount(),
                currency: w.currency().to_string(),
            }),
        }
    }
}

impl Converter<DeleteMethod> for ConverterImpl<DeleteMethod> {
    type Req = DeleteRequest;
    type Res = DeleteResponse;
    type InputData = DeleteInputData;
    type OutputData = DeleteOutputData;

    fn decode(req: &DeleteRequest) -> DeleteInputData {
        DeleteInputData{
            id: req.id.clone(),
        }
    }

    fn encode(_: DeleteOutputData) -> DeleteResponse {
        DeleteResponse{}
    }
}
