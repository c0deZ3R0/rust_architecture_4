pub mod error;

use super::error::Error as InMemoryRepositoryError;
use crate::result::*;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::entity::traits::Identifiable;

use super::repository::Repository;

// Assuming Identifiable is defined elsewhere and suitable for async contexts
pub struct InMemoryRepository<T: Identifiable> {
    data: Arc<Mutex<HashMap<Uuid, T>>>, // Using Arc<Mutex<>> for thread-safe async access
}

impl<T: Identifiable + Clone + Send + Sync> InMemoryRepository<T> {
    // Ensure T is Send + Sync for async
    pub fn new() -> Self {
        InMemoryRepository {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl<T: Identifiable + Clone + Send + Sync + 'static> Repository<T> for InMemoryRepository<T> {
    async fn add(&self, item: T) -> Result<Uuid> {
        let mut data = self.data.lock().await;
        let id = item.id();
        if data.contains_key(&id) {
            Err(Box::new(InMemoryRepositoryError::BasicError {
                message: "Item already exists".into(),
            }))
        } else {
            data.insert(id, item);
            Ok(id)
        }
    }

    async fn get(&self, id: Uuid) -> Result<Option<T>> {
        let data = self.data.lock().await;
        Ok(data.get(&id).cloned())
    }

    async fn update(&self, item: T) -> Result<()> {
        let mut data = self.data.lock().await;
        let id = item.id();
        if data.contains_key(&id) {
            data.insert(id, item);
            Ok(())
        } else {
            Err(Box::new(InMemoryRepositoryError::BasicError {
                message: "Item not found".into(),
            }))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut data = self.data.lock().await;
        if data.remove(&id).is_some() {
            Ok(())
        } else {
            Err(Box::new(InMemoryRepositoryError::BasicError {
                message: "Item already exists".into(),
            }))
        }
    }
}
