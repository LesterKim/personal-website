mod application;
mod domain;
mod infrastructure;
mod ports;

use axum::{
    routing::get,
    Router,
    extract::Path,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create our application router
    let app = Router::new()
        .route("/", get(infrastructure::handlers::home::handle_home))
        .route("/bio", get(infrastructure::handlers::bio::handle_bio))
        .route("/services", get(infrastructure::handlers::services::handle_services))
        .route("/blog", get(infrastructure::handlers::blog::handle_blog))
        .route("/blog/:id", get(infrastructure::handlers::blog::handle_blog_post))
        .nest_service("/static", ServeDir::new("static"));

    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
