// service/section_service.rs
use crate::result::*;
use crate::service::error::Error as ServiceError;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use chrono::format::Item;
use std::fmt::format;
use uuid::Uuid;

use crate::entity::section::Section;

use super::super::repository::repository::Repository;
use super::generic_service::GenericService;

impl GenericService<Section> {
    pub fn new(repo: Arc<Mutex<dyn Repository<Section> + Send + Sync>>) -> Self {
        GenericService { repository: repo }
    }

    // Make `with_repository` async to properly await lock acquisition and repo operations
    async fn with_repository<F, Fut, R>(&self, operation: F) -> Result<R>
    where
        F: FnOnce(&mut (dyn Repository<Section> + Send + Sync + 'static)) -> Fut + Send,
        Fut: Future<Output = Result<R>> + Send,
        R: Send + 'static,
    {
        let mut repo = self.repository.lock().await;
        operation(&mut *repo).await
    }

    pub async fn add_section(&self, section: Section) -> Result<Section> {
        // Assuming is_valid_section is a synchronous function validating the section
        // This needs to be defined and should return a Result<(), Error>
        Self::is_valid_section(&section)?;

        // Acquire a lock and attempt to add the section to the repository
        let mut repo = self.repository.lock().await;

        let operation_result = repo.add(section.clone()).await;

        operation_result.map_err(|err| ServiceError::LockError {
            message: format!("Error adding section: {}", err),
        })?;

        Ok(section)
    }

    pub async fn get_section(&self, id: Uuid) -> Result<Option<Section>> {
        // Acquire a lock and attempt to retrieve the section from the repository
        let repo = self.repository.lock().await;
        Ok(repo.get(id).await.map_err(|err| ServiceError::LockError {
            message: format!("Error getting section: {}", err),
        })?)
    }

    pub async fn update_section(&self, section: Section) -> Result<Section> {
        // Assuming is_valid_section is a synchronous function validating the section
        // This needs to be defined and should return a Result<(), Error>
        Self::is_valid_section(&section)?;

        // Acquire a lock and attempt to update the section in the repository
        let mut repo = self.repository.lock().await;
        repo.update(section.clone())
            .await
            .map_err(|err| ServiceError::LockError {
                message: format!("Error updating section: {}", err),
            })?;

        Ok(section)
    }

    pub async fn delete_section(&self, id: Uuid) -> Result<()> {
        // Acquire a lock and attempt to delete the section from the repository
        let mut repo = self.repository.lock().await;
        repo.delete(id)
            .await
            .map_err(|err| ServiceError::LockError {
                message: format!("Error deleting section: {}", err),
            })?;
        Ok(())
    }

    fn is_valid_section(section: &Section) -> Result<()> {
        //Section::is_valid_name(&section.name)?;
        //Section::is_valid_description(&section.description)?;
        Ok(())
    }
}
