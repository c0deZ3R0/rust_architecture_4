// dto/estimate_dto.rs

use uuid::Uuid;

use crate::entity::estimate::Estimate;

#[derive(Debug, Clone)]
pub struct EstimateDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub location: String,
}

impl EstimateDTO {
    pub fn new() -> Self {
        EstimateDTO {
            id: Uuid::new_v4(),
            name: "".to_string(),
            description: "".to_string(),
            location: "".to_string(),
        }
    }
}

impl From<Estimate> for EstimateDTO {
    fn from(estimate: Estimate) -> Self {
        EstimateDTO {
            id: estimate.id,
            name: estimate.name,
            description: estimate.description,
            location: estimate.location,
        }
    }
}

impl From<EstimateDTO> for Estimate {
    fn from(estimate_dto: EstimateDTO) -> Self {
        Estimate {
            id: estimate_dto.id,
            name: estimate_dto.name,
            description: estimate_dto.description,
            price: 0.0,
            location: "".to_string(),
            price_guess: 0.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_to_dto_conversion() {
        // Create an example Estimate object
        let estimate = Estimate {
            id: Uuid::new_v4(),
            name: "Test Estimate".to_string(),
            description: "Description".to_string(),
            price: 10.0,
            location: "Location".to_string(),
            price_guess: 10.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Convert the Estimate to an EstimateDTO
        let estimate_dto = EstimateDTO::from(estimate.clone());

        // Assert that the conversion preserves the essential fields
        assert_eq!(estimate_dto.id, estimate.id);
        assert_eq!(estimate_dto.name, estimate.name);
        assert_eq!(estimate_dto.description, estimate.description);
        assert_eq!(estimate_dto.location, estimate.location);
    }

    #[test]
    fn test_estimate_dto_to_estimate_conversion() {
        // Create an example EstimateDTO object
        let estimate_dto = EstimateDTO::new();

        // Convert the EstimateDTO to an Estimate
        let estimate = Estimate::from(estimate_dto.clone());

        // Assert that the conversion preserves the essential fields
        assert_eq!(estimate.id, estimate_dto.id);
        assert_eq!(estimate.name, estimate_dto.name);
        assert_eq!(estimate.description, estimate_dto.description);
        assert_eq!(estimate.price, 0.0); // Price defaults to 0.0 in the conversion
        assert_eq!(estimate.location, "".to_string()); // Location defaults to an empty string
        assert_eq!(estimate.price_guess, 0.0); // Price guess defaults to 0.0 in the conversion
    }
}
