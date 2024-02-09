//use_case/error.rs

use crate::error::Error as MainError;
use crate::result::*;

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
pub enum Error {
    #[display("UseCaseInnerError: {}", message)]
    BasicCaseError { message: String },
}

impl std::error::Error for Error {}

impl From<Error> for MainError {
    fn from(error: Error) -> Self {
        match error {
            Error::BasicCaseError { message } => {
                MainError::UseCaseError(Error::BasicCaseError { message }.into())
            }
        }
    }
}
