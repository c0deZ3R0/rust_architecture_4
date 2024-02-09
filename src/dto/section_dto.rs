use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entity::section::Section;

#[derive(Debug, Clone, PartialEq)]
pub struct SectionDTO {
    id: Uuid,
    pub name: String,
    pub code: String,

    pub description: Option<String>,
    pub sections: Option<Vec<Section>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,

    pub estimate_id: Option<Uuid>,
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.code == other.code
            && self.name == other.name
            && self.description == other.description
            && self.sections == other.sections
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at
            && self.estimate_id == other.estimate_id
    }
}

impl SectionDTO {
    pub fn new(name: String, code: String) -> Self {
        SectionDTO {
            id: Uuid::new_v4(),
            code: code,
            name: name,
            description: None,
            sections: None,
            estimate_id: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }

    pub fn get_updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }
}

impl From<Section> for SectionDTO {
    fn from(section: Section) -> Self {
        SectionDTO {
            id: section.id,
            code: section.code,
            name: section.name,
            description: Some(section.description),
            sections: Some(section.sections),
            estimate_id: section.estimate_id,
            created_at: Some(section.created_at),
            updated_at: Some(section.updated_at),
        }
    }
}

impl From<SectionDTO> for Section {
    fn from(section_dto: SectionDTO) -> Self {
        Section {
            id: section_dto.id,
            code: section_dto.code,
            name: section_dto.name,
            description: section_dto.description.unwrap_or("".to_string()),
            sections: section_dto.sections.unwrap_or(vec![]),
            created_at: section_dto.created_at.unwrap_or(chrono::Utc::now()),
            updated_at: section_dto.created_at.unwrap_or(chrono::Utc::now()),
            estimate_id: None,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_section_to_dto_conversion() {
        // Create an example Section object
        let section = Section {
            id: Uuid::new_v4(),
            code: "Test Section".to_string(),
            name: "Name".to_string(),
            description: "Description".to_string(),
            sections: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            estimate_id: None,
        };

        // Convert the Section object to a SectionDTO
        let section_dto = SectionDTO::from(section.clone());

        // Check that the conversion was successful
        assert_eq!(section.id, section_dto.id);
        assert_eq!(section.code, section_dto.code);
        assert_eq!(section.name, section_dto.name);
        assert_eq!(section.description, section_dto.description.unwrap());
        assert_eq!(section.sections, section_dto.sections.unwrap());
        assert_eq!(section.created_at, section_dto.created_at.unwrap());
        assert_eq!(section.updated_at, section_dto.updated_at.unwrap());
    }

    #[test]
    fn test_dto_to_section_conversion() {
        // Create an example SectionDTO object
        let section_dto = SectionDTO::new("Test Section".to_string(), "Code".to_string());
        println!("{:?}", section_dto);
        // Convert the SectionDTO object to a Section
        let section = Section::from(section_dto.clone());

        // Check that the conversion was successful
        assert_eq!(section_dto.id, section.id);
        assert_eq!(section_dto.code, section.code);
        assert_eq!(section_dto.name, section.name);
    }
}
