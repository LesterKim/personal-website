use axum::response::Html;
use askama::Template;
use chrono::Datelike;
use crate::application::services::ServiceCatalog;
use crate::infrastructure::repositories::mock::MockServiceRepository;

#[derive(Template)]
#[template(path = "services.html")]
struct ServicesTemplate {
    year: i32,
    services: Vec<Service>,
}

struct Service {
    name: String,
    description: String,
    price_range: String,
}

pub async fn handle_services() -> Html<String> {
    let service_catalog = ServiceCatalog::new(MockServiceRepository::new());
    let domain_services = service_catalog.get_services().await.unwrap();

    let services: Vec<Service> = domain_services
        .into_iter()
        .map(|service| Service {
            name: service.name,
            description: service.description,
            price_range: service.price_range,
        })
        .collect();

    let template = ServicesTemplate {
        year: chrono::Local::now().year(),
        services,
    };
    Html(template.render().unwrap())
} 