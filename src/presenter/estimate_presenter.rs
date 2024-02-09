//presenter/estimate_presenter.rs
use super::error::Error as PresenterError;
use crate::result::*;

use crate::controller::estimate_controller::CreateEstimateResponse;

pub struct EstimatePresenter;

impl EstimatePresenter {
    pub fn present(response: CreateEstimateResponse) -> Result<String> {
        match response.status_code {
            200 => Ok(format!("Success: {}", response.message)),
            _ => Err(Box::new(PresenterError::BasicError {
                status_code: response.status_code,
                message: response.message,
            })),
        }
    }
}
