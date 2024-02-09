//entity/error.rs
use crate::error::Error as MainError;

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
pub enum Error {
    #[display("Validation error in {}: {}", entity, message)]
    ValidationError {
        entity: &'static str,
        message: String,
    },
}

impl std::error::Error for Error {}

impl From<Error> for MainError {
    fn from(error: Error) -> Self {
        match error {
            Error::ValidationError { entity, message } => {
                MainError::EntityError(Error::ValidationError { entity, message }.into())
            }
        }
    }
}
