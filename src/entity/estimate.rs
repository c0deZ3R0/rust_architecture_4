// entity/estimate.rs

use crate::result::*;

use super::error::Error;
use super::error::Error as EntityError;
use super::traits::Identifiable;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Estimate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub location: String,
    pub price_guess: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Identifiable for Estimate {
    fn id(&self) -> Uuid {
        self.id
    }
}

// region:    --- Basic Estimate Validation Rules

impl Estimate {
    pub fn is_valid_name(name: &str) -> Result<()> {
        let validated = name.len() >= 50 && name.len() <= 100;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Estimate",
                message: "Name must be between 3 and 100 characters".into(),
            }))
        }
    }

    pub fn is_valid_description(description: &str) -> Result<()> {
        let validated = description.len() >= 1 && description.len() <= 1000;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Estimate",
                message: "Description must be between 1 and 1000 characters".into(),
            }))
        }
    }

    pub fn is_valid_price(price: f64) -> Result<()> {
        let validated = price > 0.0;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Estimate",
                message: "Price must be greater than 0".into(),
            }))
        }
    }

    pub fn is_valid_location(location: &str) -> Result<()> {
        let validated = location.len() >= 3 && location.len() <= 100;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Estimate",
                message: "Location must be between 3 and 100 characters".into(),
            }))
        }
    }

    pub fn is_valid_price_guess(price_guess: f64) -> Result<()> {
        let validated = price_guess > 0.0;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Estimate",
                message: "Price guess must be greater than 0".into(),
            }))
        }
    }
}

// endregion: --- Basic Estimate Validation Rules

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        // Test a valid name
        assert!(Estimate::is_valid_name("Valid Name").is_ok());

        // Test a name shorter than 3 characters
        assert!(Estimate::is_valid_name("A").is_err());

        // Test a name longer than 100 characters
        let long_name = "a".repeat(101);
        assert!(Estimate::is_valid_name(&long_name).is_err());
    }

    #[test]
    fn test_valid_description() {
        // Test a valid description
        assert!(Estimate::is_valid_description("Valid Description").is_ok());

        // Test a description shorter than 10 characters
        assert!(Estimate::is_valid_description("Short").is_err());

        // Test a description longer than 1000 characters
        let long_description = "a".repeat(1001);
        assert!(Estimate::is_valid_description(&long_description).is_err());
    }

    #[test]
    fn test_valid_price() {
        // Test a valid price
        assert!(Estimate::is_valid_price(10.0).is_ok());

        // Test a price of 0
        assert!(Estimate::is_valid_price(0.0).is_err());

        // Test a negative price
        assert!(Estimate::is_valid_price(-10.0).is_err());
    }

    #[test]
    fn test_valid_location() {
        // Test a valid location
        assert!(Estimate::is_valid_location("Valid Location").is_ok());

        // Test a location shorter than 3 characters
        assert!(Estimate::is_valid_location("A").is_err());

        // Test a location longer than 100 characters
        let long_location = "a".repeat(101);
        assert!(Estimate::is_valid_location(&long_location).is_err());
    }

    #[test]
    fn test_valid_price_guess() {
        // Test a valid price guess
        assert!(Estimate::is_valid_price_guess(10.0).is_ok());

        // Test a price guess of 0
        assert!(Estimate::is_valid_price_guess(0.0).is_err());

        // Test a negative price guess
        assert!(Estimate::is_valid_price_guess(-10.0).is_err());
    }
}
