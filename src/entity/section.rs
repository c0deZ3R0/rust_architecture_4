//entity/section.rs

use super::error::Error as EntityError;
use crate::result::*;

use super::traits::Identifiable;

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Section {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: String,
    pub sections: Vec<Section>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub estimate_id: Option<Uuid>,
}

impl Identifiable for Section {
    fn id(&self) -> Uuid {
        self.id
    }
}

// region:    --- Basic Section Validation Rules

impl Section {
    pub fn is_valid_name(name: &str) -> Result<()> {
        let validated = name.len() >= 3 && name.len() <= 100;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Section",
                message: "Name must be between 3 and 100 characters".into(),
            }))
        }
    }

    pub fn is_valid_description(description: &str) -> Result<()> {
        let validated = description.len() >= 10 && description.len() <= 1000;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Section",
                message: "Description must be between 10 and 1000 characters".into(),
            }))
        }
    }

    pub fn is_valid_sections(sections: &Vec<Section>) -> Result<()> {
        let validated = sections.len() <= 100;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Section",
                message: "Sections must be less than 100".into(),
            }))
        }
    }

    pub fn is_valid_project_id(project_id: &Uuid) -> Result<()> {
        let validated = project_id.to_string().len() > 0;

        if validated {
            Ok(())
        } else {
            Err(Box::new(EntityError::ValidationError {
                entity: "Section",
                message: "Project ID must be a valid UUID".into(),
            }))
        }
    }
}

// endregion: --- Basic Section Validation Rules
