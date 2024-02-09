// use_case/create_section_add_to_estimate.rs
use crate::result::*;
use crate::use_case::error::Error as UseCaseError;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;

use crate::dto::estimate_dto::EstimateDTO;
use crate::dto::section_dto::SectionDTO;
use crate::entity::estimate::Estimate;
use crate::entity::section::{self, Section};

use crate::service::generic_service::{GenericService, Service};

pub struct CreateSectionAddToEstimate {
    section_service: Arc<Mutex<GenericService<Section>>>,
    estimate_service: Arc<Mutex<GenericService<Estimate>>>,
}

impl CreateSectionAddToEstimate {
    pub fn new(
        section_service: Arc<Mutex<GenericService<Section>>>,
        estimate_service: Arc<Mutex<GenericService<Estimate>>>,
    ) -> Self {
        CreateSectionAddToEstimate {
            section_service,
            estimate_service,
        }
    }

    pub async fn execute(
        &self,
        section_dto: SectionDTO,
        estimate_dto: EstimateDTO,
    ) -> Result<Uuid> {
        // Convert DTOs to entities
        let estimate = Estimate::from(estimate_dto);
        let mut section = Section::from(section_dto);

        // Acquire lock asynchronously on estimate service and check if the estimate exists
        let estimate_service = self.estimate_service.lock().await;
        match estimate_service.get_estimate(estimate.id).await {
            Ok(Some(_)) => {
                // If the estimate exists, proceed to add the section
                section.estimate_id = Some(estimate.id);
                let section_service = self.section_service.lock().await;
                match section_service.add_section(section).await {
                    Ok(section) => Ok(section.id),
                    Err(e) => Err(e.into()), // Properly convert or handle the error here
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
