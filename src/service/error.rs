//service/error.rs

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::error::Error as MainError;

#[serde_as]
#[derive(Debug, Serialize, Display)]
pub enum Error {
    #[display("Validation error: {}", message)]
    ValidationError { message: String },

    #[display("Lock error: {}", message)]
    LockError { message: String },
}

impl std::error::Error for Error {}

impl From<Error> for MainError {
    fn from(error: Error) -> Self {
        match error {
            Error::ValidationError { message } => {
                MainError::ServiceError(Error::ValidationError { message }.into())
            }
            Error::LockError { message } => {
                MainError::ServiceError(Error::LockError { message }.into())
            }
        }
    }
}
