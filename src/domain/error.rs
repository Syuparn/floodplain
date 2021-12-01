use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum WalletError {
    #[error("currency unit `{0}` is invalid")]
    InvalidCurrency(String),
    #[error("unexpected error: `{0}")]
    Unexpected(String),
    #[error("unknown error")]
    Unknown,
}
