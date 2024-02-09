use crate::result::*;

use derive_more::{Display, From};
use serde::Serialize;
use serde_json::error;
use std::fmt;

// Assuming each module's error implements std::error::Error and std::fmt::Display
use crate::{controller, entity, presenter, repository, service, use_case};

// Unified Result type for the entire crate

#[derive(Debug, Serialize, Display)]
pub enum Error {
    #[display("Main thread error: {}", message)]
    MainError { message: String },
    #[display("Repository error: {}", _0)]
    RepositoryError(repository::error::Error),
    #[display("InMemoryRepository error: {}", _0)]
    InMemoryRepositoryError(repository::in_memory_repo::error::Error),
    #[display("UseCase error: {}", _0)]
    UseCaseError(use_case::error::Error),
    #[display("Entity error: {}", _0)]
    EntityError(entity::error::Error),
    #[display("Service error: {}", _0)]
    ServiceError(service::error::Error),
    #[display("Controller error: {}", _0)]
    ControllerError(controller::error::Error),
    #[display("Presenter error: {}", _0)]
    PresenterError(presenter::error::Error),
    // MyError could be included if it's used outside as a common error type
    // MyError(MyError),
}

// This struct and its impls can be included if you need a specific error type
// that doesn't fit the existing categories

impl std::error::Error for Error {}

impl From<Error> for Result<()> {
    fn from(error: Error) -> Self {
        Err(error.into())
    }
}

#[macro_export]
macro_rules! try_or_log {
    ($expr:expr, $success_msg:expr, $err_msg:expr) => {{
        let result = $expr;
        match &result {
            Ok(_) => println!("Success: {}", $success_msg),
            Err(_) => println!("Error: {}", $err_msg),
        }
        result?
    }};
}
