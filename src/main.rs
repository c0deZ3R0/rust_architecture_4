//main.rs

use crate::error::Error;

mod controller;
mod dto;
mod entity;
mod error;
mod presenter;
mod repository;
mod result;
mod service;
mod use_case;

use std::sync::Arc;

use dto::estimate_dto::EstimateDTO;
use entity::{estimate::Estimate, section::Section};

use repository::in_memory_repo::InMemoryRepository;
use service::generic_service::GenericService;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::dto::section_dto::SectionDTO;

type Result<T> = std::result::Result<T, Box<error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the repository for entities.
    let estimate_repo = Arc::new(Mutex::new(InMemoryRepository::<Estimate>::new()));
    let section_repo = Arc::new(Mutex::new(InMemoryRepository::<Section>::new()));

    // Initialize the services with the repositories
    let estimate_service = Arc::new(Mutex::new(GenericService::<Estimate>::new(estimate_repo)));
    let section_service = Arc::new(Mutex::new(GenericService::<Section>::new(section_repo)));

    // Initialize the use cases with the services
    let create_estimate =
        use_case::create_estimate::CreateEstimate::new(Arc::clone(&estimate_service));
    let create_section_add_to_estimate =
        use_case::create_section_add_to_estimate::CreateSectionAddToEstimate::new(
            Arc::clone(&section_service),
            Arc::clone(&estimate_service),
        );

    // Initialize the controllers with the use cases
    let section_controller =
        controller::section_controller::SectionController::new(create_section_add_to_estimate);
    let estimate_controller =
        controller::estimate_controller::EstimateController::new(create_estimate);

    // Create a new estimate
    let mut new_estimate_dto = dto::estimate_dto::EstimateDTO::new();
    new_estimate_dto.name = "New Estimate".to_string();
    new_estimate_dto.description = "A new estimate".to_string();
    new_estimate_dto.location = "New York".to_string();

    println!("Creating new estimate: {:?}", new_estimate_dto);

    let estimate_request = controller::estimate_controller::CreateEstimateRequest::new(
        new_estimate_dto.name,
        new_estimate_dto.description,
        new_estimate_dto.location,
    );

    let estimate_id_result = estimate_controller.create_estimate(estimate_request).await;
    match estimate_id_result {
        Ok(estimate_id) => {
            println!("Estimate created successfully with ID: {:?}", estimate_id);
        }
        Err(e) => {
            return Err(Box::new(Error::MainError {
                message: format!("Error creating estimate: {}", e),
            }));
        }
    }

    let retrieved_estimate_dto: Result<EstimateDTO> = match estimate_id_result {
        Ok(estimate_id) => {
            let estimate_service = estimate_service.lock().await;
            let estimate = estimate_service.get_estimate(estimate_id).await;
            match estimate {
                Ok(Some(estimate)) => Ok(estimate.into()),
                Ok(None) => Err(Box::new(Error::MainError {
                    message: "Estimate does not exist".to_string(),
                })),
                Err(e) => Err(Box::new(Error::MainError {
                    message: format!("Error getting estimate: {}", e),
                })),
            }
        }
        Err(e) => Err(Box::new(Error::MainError {
            message: format!("Error creating estimate: {}", e),
        })),
    };

    let mut new_section_dto =
        dto::section_dto::SectionDTO::new("New Section".to_string(), "Code:0001".to_string());

    let section_request = match retrieved_estimate_dto {
        Ok(estimate_dto) => {
            println!("Retrieved estimate: {:?}", estimate_dto);
            // Construct the request using the successfully retrieved estimate_dto.
            Some(
                controller::section_controller::CreateSectionAddToEstimateRequest::new(
                    new_section_dto,
                    estimate_dto,
                ),
            )
        }
        Err(e) => {
            return Err(Box::new(Error::MainError {
                message: "Error retrieving estimate DTO".to_string(),
            }));
        }
    };

    if let Some(request) = section_request {
        // Proceed with creating the section, assuming this function call is async and returns a Result.
        let section_result = section_controller
            .create_section_add_to_estimate(request)
            .await;
        println!("Section Results --> {:?}", section_result);

        match section_result {
            Ok(section_id) => {
                // Handle the successful creation of the section, if necessary.
                println!("Section created successfully with ID: {:?}", section_id);

                // Attempt to retrieve the created section
                let section = section_service.lock().await.get_section(section_id).await;

                match section {
                    Ok(t) => println!("Section retrieved successfully: {:?}", t),
                    Err(e) => {
                        // Return an error if retrieving the section fails
                        return Err(Box::new(Error::MainError {
                            message: format!("Error retrieving section: {}", e),
                        }));
                    }
                }
            }
            Err(e) => {
                // Return an error if creating the section fails
                return Err(Box::new(Error::MainError {
                    message: format!("Error creating section: {}", e),
                }));
            }
        }
    } else {
        // Return an error if section_request is None
        return Err(Box::new(Error::MainError {
            message: "Section request is None".to_string(),
        }));
    };

    Ok(())
}
