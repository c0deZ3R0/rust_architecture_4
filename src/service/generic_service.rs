// service/generic_service.rs

use crate::result::*;

use async_trait::async_trait; // 0.1.50, for async trait methods
use std::future::Future;
use tokio::sync::Mutex;
// 1.15.0, for async-friendly mutex
use std::sync::Arc;
use uuid::Uuid;

use super::super::repository::repository::Repository;
use crate::entity::traits::Identifiable;

pub struct GenericService<T> {
    pub repository: Arc<Mutex<dyn Repository<T> + Send + Sync>>,
}

#[async_trait]
pub trait Service<T>
where
    T: Clone + Send + Sync,
{
    async fn with_repository<F, Fut, R>(&self, operation: F) -> Result<R>
    where
        F: FnOnce(&mut (dyn Repository<T> + Send + Sync + 'static)) -> Fut + Send,
        Fut: Future<Output = Result<R>> + Send,
        R: Send + 'static;

    async fn add(&self, item: T) -> Result<Uuid>;
    async fn get(&self, id: Uuid) -> Result<Option<T>>;
    async fn update(&self, item: T) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
impl<T> Service<T> for GenericService<T>
where
    T: Identifiable + Clone + Send + Sync + 'static, // Adjust trait bounds for async and concurrency
{
    async fn add(&self, item: T) -> Result<Uuid> {
        let mut repo = self.repository.lock().await; // Use async lock
        repo.add(item).await // Assume repo.add is async
    }

    async fn get(&self, id: Uuid) -> Result<Option<T>> {
        let repo = self.repository.lock().await;
        repo.get(id).await // Assume repo.get is async
    }

    async fn update(&self, item: T) -> Result<()> {
        let mut repo = self.repository.lock().await;
        repo.update(item).await // Assume repo.update is async
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut repo = self.repository.lock().await;
        repo.delete(id).await // Assume repo.delete is async
    }

    async fn with_repository<F, Fut, R>(&self, operation: F) -> Result<R>
    where
        F: FnOnce(&mut (dyn Repository<T> + Send + Sync + 'static)) -> Fut + Send,
        Fut: Future<Output = Result<R>> + Send,
        R: Send + 'static,
    {
        let mut repo = self.repository.lock().await;
        operation(&mut *repo).await
    }
}
