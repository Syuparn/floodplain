use mockall::{automock, predicate::*};

use crate::domain::error::WalletError;

#[automock]
pub trait Port<In, Out> {
    fn exec(&self, input: In) -> Result<Out, WalletError>;
}
