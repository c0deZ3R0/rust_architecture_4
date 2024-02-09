//repository/error.rs
use crate::error::Error as MainError;

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use std::borrow::Cow;

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
pub enum Error {
    #[display("Placeholder error: {}", message)]
    BasicError { message: String },
}

impl std::error::Error for Error {}

impl From<Error> for MainError {
    fn from(error: Error) -> Self {
        match error {
            Error::BasicError { message } => {
                MainError::RepositoryError(Error::BasicError { message }.into())
            }
        }
    }
}
