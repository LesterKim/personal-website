use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::models::{BlogPost, Service};
use crate::ports::repositories::{BlogRepository, ServiceRepository, Error};

pub struct MockBlogRepository;

impl MockBlogRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BlogRepository for MockBlogRepository {
    async fn get_posts(&self) -> Result<Vec<BlogPost>, Error> {
        Ok(vec![
            BlogPost {
                id: "1".to_string(),
                title: "Building Scalable Web3 Infrastructure".to_string(),
                content: "A deep dive into the architecture patterns and best practices for building scalable blockchain applications using Rust and WebAssembly...".to_string(),
                published_at: DateTime::parse_from_rfc3339("2024-11-30T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            },
            BlogPost {
                id: "2".to_string(),
                title: "The Future of DeFi: A Technical Perspective".to_string(),
                content: "Exploring the technical challenges and opportunities in decentralized finance, with a focus on security and performance optimization...".to_string(),
                published_at: DateTime::parse_from_rfc3339("2024-11-25T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            },
        ])
    }

    async fn get_post(&self, id: &str) -> Result<Option<BlogPost>, Error> {
        match id {
            "1" => Ok(Some(BlogPost {
                id: "1".to_string(),
                title: "Building Scalable Web3 Infrastructure".to_string(),
                content: "A deep dive into the architecture patterns and best practices for building scalable blockchain applications using Rust and WebAssembly...".to_string(),
                published_at: DateTime::parse_from_rfc3339("2024-11-30T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            })),
            "2" => Ok(Some(BlogPost {
                id: "2".to_string(),
                title: "The Future of DeFi: A Technical Perspective".to_string(),
                content: "Exploring the technical challenges and opportunities in decentralized finance, with a focus on security and performance optimization...".to_string(),
                published_at: DateTime::parse_from_rfc3339("2024-11-25T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            })),
            _ => Ok(None),
        }
    }
}

pub struct MockServiceRepository;

impl MockServiceRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceRepository for MockServiceRepository {
    async fn get_services(&self) -> Result<Vec<Service>, Error> {
        Ok(vec![
            Service {
                id: "1".to_string(),
                name: "Technical Leadership".to_string(),
                description: "Strategic technical guidance for your startup's growth and success.".to_string(),
                price_range: "$200-500/hour".to_string(),
            },
            Service {
                id: "2".to_string(),
                name: "Web3 Development".to_string(),
                description: "End-to-end development of decentralized applications.".to_string(),
                price_range: "Custom pricing".to_string(),
            },
        ])
    }
} 