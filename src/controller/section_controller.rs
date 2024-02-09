//controller/section_controller.rs

use crate::dto::estimate_dto::EstimateDTO;
use crate::dto::section_dto::SectionDTO;
use crate::entity::section::Section;
use crate::result::*;
use uuid::Uuid;

use crate::use_case::create_section_add_to_estimate::CreateSectionAddToEstimate;
use crate::use_case::error::Error;

pub struct SectionController {
    create_section_add_to_estimate: CreateSectionAddToEstimate,
}

impl SectionController {
    pub fn new(create_section_add_to_estimate: CreateSectionAddToEstimate) -> SectionController {
        SectionController {
            create_section_add_to_estimate,
        }
    }

    pub async fn create_section_add_to_estimate(
        &self,
        request: CreateSectionAddToEstimateRequest,
    ) -> Result<Uuid> {
        let result = self
            .create_section_add_to_estimate
            .execute(request.section, request.estimate)
            .await;

        match result {
            Ok(section) => CreateSectionAddToEstimateResponse::new(
                200,
                "Section created successfully".to_string(),
                Some(section),
            ),
            Err(_) => CreateSectionAddToEstimateResponse::new(500, "E".to_string(), None),
        };

        result
    }
}

pub struct CreateSectionAddToEstimateRequest {
    pub section: SectionDTO,
    pub estimate: EstimateDTO,
}

impl CreateSectionAddToEstimateRequest {
    pub fn new(section: SectionDTO, estimate: EstimateDTO) -> CreateSectionAddToEstimateRequest {
        CreateSectionAddToEstimateRequest { section, estimate }
    }
}

pub struct CreateSectionAddToEstimateResponse {
    pub status: u16,
    pub message: String,
    pub section: Option<Uuid>,
}

impl CreateSectionAddToEstimateResponse {
    pub fn new(
        status: u16,
        message: String,
        section: Option<Uuid>,
    ) -> CreateSectionAddToEstimateResponse {
        CreateSectionAddToEstimateResponse {
            status,
            message,
            section,
        }
    }
}
