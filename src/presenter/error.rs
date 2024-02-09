//presenter/error.rs
use crate::error::Error as MainError;

use derive_more::{Display, From};

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, From, Display)]
pub enum Error {
    #[display("Placeholder error: {}", message)]
    BasicError { status_code: u16, message: String },
}

impl std::error::Error for Error {}

impl From<Error> for MainError {
    fn from(error: Error) -> Self {
        match error {
            Error::BasicError {
                status_code,
                message,
            } => MainError::PresenterError(
                Error::BasicError {
                    status_code,
                    message,
                }
                .into(),
            ),
        }
    }
}
