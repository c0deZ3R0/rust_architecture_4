use crate::result::*;
use crate::service::error::Error as ServiceError;

use crate::entity::estimate::Estimate;
use crate::entity::section::Section;
use crate::repository::repository::Repository; // Adjust path as necessary
use crate::service::generic_service::GenericService;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

impl GenericService<Estimate> {
    pub fn new(repo: Arc<Mutex<dyn Repository<Estimate> + Send + Sync>>) -> Self {
        GenericService { repository: repo }
    }

    // Make `with_repository` async to properly await lock acquisition and repo operations
    async fn with_repository<F, Fut, R>(&self, operation: F) -> Result<R>
    where
        F: FnOnce(&mut (dyn Repository<Estimate> + Send + Sync + 'static)) -> Fut + Send,
        Fut: Future<Output = Result<R>> + Send,
        R: Send + 'static,
    {
        let mut repo = self.repository.lock().await;
        operation(&mut *repo).await
    }

    pub async fn add_estimate(&self, estimate: Estimate) -> Result<Estimate> {
        // Assuming is_valid_estimate is a synchronous function validating the estimate
        // This needs to be defined and should return a Result<(), Error>
        let is_valid = Self::is_valid_estimate(&estimate);

        if let Err(e) = is_valid {
            return Err(e);
        }
        // Acquire a lock and attempt to add the estimate to the repository
        let mut repo = self.repository.lock().await;

        let operation_result = repo.add(estimate.clone()).await;

        operation_result.map_err(|err| ServiceError::LockError {
            message: format!("Error adding estimate: {}", err),
        })?;

        Ok(estimate)
    }

    pub async fn get_estimate(&self, id: Uuid) -> Result<Option<Estimate>> {
        // Acquire a lock and attempt to retrieve the estimate from the repository
        let repo = self.repository.lock().await;
        Ok(repo.get(id).await.map_err(|err| ServiceError::LockError {
            message: format!("Error getting estimate: {}", err),
        })?)
    }

    pub async fn update_estimate(&self, estimate: Estimate) -> Result<Estimate> {
        // Assuming is_valid_estimate is a synchronous function validating the estimate
        // This needs to be defined and should return a Result<(), Error>
        Self::is_valid_estimate(&estimate)?;

        // Acquire a lock and attempt to update the estimate in the repository
        let mut repo = self.repository.lock().await;
        repo.update(estimate.clone())
            .await
            .map_err(|err| ServiceError::LockError {
                message: format!("Error updating estimate: {}", err),
            })?;

        Ok(estimate)
    }

    pub async fn delete_estimate(&self, id: Uuid) -> Result<()> {
        // Acquire a lock and attempt to delete the estimate from the repository
        let mut repo = self.repository.lock().await;
        repo.delete(id)
            .await
            .map_err(|err| ServiceError::LockError {
                message: format!("Error deleting estimate: {}", err),
            })?;
        Ok(())
    }

    fn is_valid_estimate(estimate: &Estimate) -> Result<()> {
        Estimate::is_valid_name(&estimate.name)?;
        Estimate::is_valid_description(&estimate.description)?;

        Ok(())
    }

    fn has_valid_price(estimate: &Estimate) -> Result<()> {
        Estimate::is_valid_price(estimate.price)?;
        Ok(())
    }
}
