use crate::domain::error::WalletError;

pub trait Port {
    type In;
    type Out;

    fn exec(&self, input: Self::In) -> Result<Self::Out, WalletError>;
}
