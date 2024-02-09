// use_case/create_estimate.rs

use crate::result::*;
use crate::use_case::error::Error as UseCaseError;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::dto::estimate_dto::EstimateDTO;
use crate::entity::estimate::Estimate;

use crate::service::generic_service::GenericService;

pub struct CreateEstimate {
    service: Arc<Mutex<GenericService<Estimate>>>,
}

impl CreateEstimate {
    pub fn new(service: Arc<Mutex<GenericService<Estimate>>>) -> Self {
        CreateEstimate { service }
    }

    pub async fn execute(&self, estimate_dto: EstimateDTO) -> Result<Uuid> {
        // Acquire lock asynchronously without map_err
        let mut service = self.service.lock().await;

        // Now you can call async operations on service
        // Ensure these operations return Results to use map_err or ?
        match service.add_estimate(Estimate::from(estimate_dto)).await {
            Ok(estimate) => Ok(estimate.id),
            Err(e) => Err(Box::new(UseCaseError::BasicCaseError {
                message: format!("Error adding estimate: {}", e),
            })),
        }
    }
}
