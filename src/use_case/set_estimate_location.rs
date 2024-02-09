//use_case/set_estimate_location.rs
use crate::result::*;
use crate::use_case::error::Error as UseCaseError;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;

use crate::{
    dto::estimate_dto::EstimateDTO,
    entity::estimate::Estimate,
    service::generic_service::{GenericService, Service},
};

pub struct SetEstimateLocation {
    service: Arc<Mutex<GenericService<Estimate>>>,
}

impl SetEstimateLocation {
    pub fn new(service: Arc<Mutex<GenericService<Estimate>>>) -> Self {
        SetEstimateLocation { service }
    }

    pub async fn execute(&mut self, estimate_id: Uuid, estimate_dto: EstimateDTO) -> Result<()> {
        let mut service = self.service.lock().await;

        match service.get(estimate_id).await {
            Ok(Some(mut estimate)) => {
                estimate.location = estimate_dto.location;
                match service.update(estimate).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(UseCaseError::BasicCaseError {
                        message: format!("Error updating estimate: {}", e),
                    })),
                }
            }
            Ok(None) => Err(Box::new(UseCaseError::BasicCaseError {
                message: "Estimate does not exist".to_string(),
            })),
            Err(e) => Err(Box::new(UseCaseError::BasicCaseError {
                message: format!("Error getting estimate: {}", e),
            })),
        }
    }
}
