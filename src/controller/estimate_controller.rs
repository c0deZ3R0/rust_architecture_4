// controller/estimate_controller.rs

use crate::result::*;

use crate::presenter::estimate_presenter::EstimatePresenter;
use crate::use_case::create_estimate::CreateEstimate;
use uuid::Uuid;

use crate::{dto::estimate_dto::EstimateDTO, entity::estimate::Estimate};

use crate::error::Error::ControllerError;

pub struct EstimateController {
    create_estimate_use_case: CreateEstimate,
}

impl EstimateController {
    pub fn new(create_estimate_use_case: CreateEstimate) -> Self {
        EstimateController {
            create_estimate_use_case,
        }
    }

    pub async fn create_estimate(&self, request: CreateEstimateRequest) -> Result<Uuid> {
        let mut dto = EstimateDTO::new();
        dto.name = request.name;
        dto.description = request.description;

        // Ensure the execute method on the use case is async and await its result
        let result = self.create_estimate_use_case.execute(dto).await;

        // Match on the async result
        let response = match result {
            Ok(estimate_id) => CreateEstimateResponse::new(
                200,
                "Estimate created successfully".to_string(),
                Some(estimate_id),
            ),
            Err(_) => CreateEstimateResponse::new(500, "Error creating estimate".to_string(), None),
        };

        EstimatePresenter::present(response); // Modify this line based on the actual behavior of your presenter

        result
    }
}

pub struct CreateEstimateRequest {
    pub name: String,
    pub description: String,
    pub location: String,
}

impl CreateEstimateRequest {
    pub fn new(name: String, description: String, location: String) -> Self {
        CreateEstimateRequest {
            name,
            description,
            location,
        }
    }
}

#[derive(Debug)]
pub struct CreateEstimateResponse {
    pub status_code: u16,
    pub message: String,
    pub estimate_id: Option<Uuid>, // Include an ID if creation was successful
}

impl CreateEstimateResponse {
    pub fn new(status_code: u16, message: String, estimate_id: Option<Uuid>) -> Self {
        CreateEstimateResponse {
            status_code,
            message,
            estimate_id,
        }
    }
}
