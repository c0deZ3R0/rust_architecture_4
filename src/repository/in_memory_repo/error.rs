//in_memory_repo/error.rs
use crate::repository::error::Error as RepositoryError;

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
pub enum Error {
    #[display("InMemoryRepositoryError: {}", message)]
    InMemoryRepositoryError { message: String },
}

impl std::error::Error for Error {}

impl From<Error> for RepositoryError {
    fn from(error: Error) -> Self {
        match error {
            Error::InMemoryRepositoryError { message } => RepositoryError::BasicError { message },
        }
    }
}
