use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    InvalidInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    InvalidTryIntError(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    InvalidRegex(#[from] regex::Error),

    #[error("Invalid Input")]
    InvalidInput,
}
