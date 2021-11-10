use crate::domain::error::WalletError;

pub trait Port<In, Out> {
    fn exec(&self, input: In) -> Result<Out, WalletError>;
}
