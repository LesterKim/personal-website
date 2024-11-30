use async_trait::async_trait;
use crate::domain::models::{BlogPost, Service};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Not found")]
    NotFound,
}

#[async_trait]
pub trait BlogRepository {
    async fn get_posts(&self) -> Result<Vec<BlogPost>, Error>;
    async fn get_post(&self, id: &str) -> Result<Option<BlogPost>, Error>;
}

#[async_trait]
pub trait ServiceRepository {
    async fn get_services(&self) -> Result<Vec<Service>, Error>;
} 