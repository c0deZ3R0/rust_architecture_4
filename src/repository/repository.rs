use crate::result::*;

use crate::entity::traits::Identifiable;
use async_trait::async_trait; // Facilitate async trait methods
use std::sync::Arc;
use tokio::sync::Mutex; // Use Tokio's Mutex for async compatibility
use uuid::Uuid;

#[async_trait] // Enables async trait methods
pub trait Repository<T>: Send
where
    T: Identifiable + Send + Sync + 'static,
{
    async fn add(&self, item: T) -> Result<Uuid>;
    async fn get(&self, id: Uuid) -> Result<Option<T>>;
    async fn update(&self, item: T) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
