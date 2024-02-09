//controller/error.rs

use crate::error::Error as MainError;
use crate::result::*;

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

use std::borrow::Cow;

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
pub enum Error {
    BasicError { message: String },
}

impl std::error::Error for Error {}

impl From<Error> for MainError {
    fn from(error: Error) -> Self {
        match error {
            Error::BasicError { message } => {
                MainError::ControllerError(Error::BasicError { message }.into())
            }
        }
    }
}
