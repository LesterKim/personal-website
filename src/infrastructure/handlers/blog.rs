use axum::response::Html;
use askama::Template;
use chrono::Datelike;
use crate::application::services::BlogService;
use crate::infrastructure::repositories::mock::MockBlogRepository;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    year: i32,
    posts: Vec<BlogPost>,
}

struct BlogPost {
    title: String,
    date: String,
    preview: String,
    slug: String,
}

pub async fn handle_blog() -> Html<String> {
    let blog_service = BlogService::new(MockBlogRepository::new());
    let domain_posts = blog_service.get_posts().await.unwrap();

    let posts: Vec<BlogPost> = domain_posts
        .into_iter()
        .map(|post| BlogPost {
            title: post.title,
            date: post.published_at.format("%B %d, %Y").to_string(),
            preview: post.content,
            slug: post.id,
        })
        .collect();

    let template = BlogTemplate {
        year: chrono::Local::now().year(),
        posts,
    };
    Html(template.render().unwrap())
} 