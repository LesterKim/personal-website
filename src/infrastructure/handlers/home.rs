use axum::response::Html;
use askama::Template;
use chrono::Datelike;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    year: i32,
}

pub async fn handle_home() -> Html<String> {
    let template = HomeTemplate {
        year: chrono::Local::now().year(),
    };
    Html(template.render().unwrap())
} 