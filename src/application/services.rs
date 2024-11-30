use crate::domain::models::{BlogPost, Service};
use crate::ports::repositories::{BlogRepository, ServiceRepository, Error};

pub struct BlogService<T: BlogRepository> {
    repository: T,
}

impl<T: BlogRepository> BlogService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn get_posts(&self) -> Result<Vec<BlogPost>, Error> {
        self.repository.get_posts().await
    }

    pub async fn get_post(&self, id: &str) -> Result<Option<BlogPost>, Error> {
        self.repository.get_post(id).await
    }
}

pub struct ServiceCatalog<T: ServiceRepository> {
    repository: T,
}

impl<T: ServiceRepository> ServiceCatalog<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn get_services(&self) -> Result<Vec<Service>, Error> {
        self.repository.get_services().await
    }
} 