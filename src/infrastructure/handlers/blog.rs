use axum::response::Html;
use axum::extract::Path;
use askama::Template;
use chrono::Datelike;
use pulldown_cmark::{Parser, Options, html};
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
    content: String,
}

pub async fn handle_blog() -> Html<String> {
    let blog_service = BlogService::new(MockBlogRepository::new());
    let domain_posts = blog_service.get_posts().await.unwrap();

    let posts: Vec<BlogPost> = domain_posts
        .into_iter()
        .map(|post| BlogPost {
            title: post.title,
            date: post.published_at.format("%B %d, %Y").to_string(),
            preview: post.content.lines().next().unwrap_or_default().to_string(),
            slug: post.id,
            content: "".to_string(), // Not needed for preview
        })
        .collect();

    let template = BlogTemplate {
        year: chrono::Local::now().year(),
        posts,
    };
    Html(template.render().unwrap())
}

pub async fn handle_blog_post(Path(id): Path<String>) -> Html<String> {
    let blog_service = BlogService::new(MockBlogRepository::new());
    
    match blog_service.get_post(&id).await.unwrap() {
        Some(post) => {
            let mut options = Options::all();
            let parser = Parser::new_ext(&post.content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            let html_content = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Your Name</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/solidity.min.js"></script>
    <script>
        document.addEventListener('DOMContentLoaded', (event) => {{
            document.querySelectorAll('pre code').forEach((el) => {{
                hljs.highlightElement(el);
            }});
        }});
    </script>
    <style>
        .prose pre {{
            background-color: #f6f8fa;
            border-radius: 6px;
            padding: 16px;
        }}
        .prose code {{
            color: #24292e;
            background-color: rgba(27,31,35,0.05);
            border-radius: 3px;
            padding: 0.2em 0.4em;
            font-size: 85%;
        }}
        .prose pre code {{
            background-color: transparent;
            padding: 0;
            font-size: 100%;
        }}
    </style>
</head>
<body class="bg-gray-50">
    <nav class="bg-white shadow-lg">
        <div class="max-w-6xl mx-auto px-4">
            <div class="flex justify-between">
                <div class="flex space-x-7">
                    <div class="flex items-center py-4">
                        <a href="/" class="text-lg font-semibold">Your Name</a>
                    </div>
                    <div class="flex items-center space-x-4">
                        <a href="/bio" class="py-4 px-2 hover:text-blue-500">Bio</a>
                        <a href="/services" class="py-4 px-2 hover:text-blue-500">Services</a>
                        <a href="/blog" class="py-4 px-2 hover:text-blue-500">Blog</a>
                    </div>
                </div>
            </div>
        </div>
    </nav>

    <main class="container mx-auto px-4 py-8">
        <div class="max-w-3xl mx-auto">
            <article class="prose lg:prose-xl">
                <header class="mb-8">
                    <h1 class="text-4xl font-bold mb-4">{}</h1>
                    <p class="text-gray-600">{}</p>
                </header>

                <div class="markdown-content">
                    {}
                </div>

                <footer class="mt-12 pt-6 border-t border-gray-200">
                    <a href="/blog" class="text-blue-600 hover:text-blue-800">← Back to Blog</a>
                </footer>
            </article>
        </div>
    </main>

    <footer class="bg-white shadow-lg mt-8">
        <div class="container mx-auto px-4 py-6 text-center">
            © {} Your Name. All rights reserved.
        </div>
    </footer>
</body>
</html>"#,
                post.title,
                post.title,
                post.published_at.format("%B %d, %Y").to_string(),
                html_output,
                chrono::Local::now().year()
            );

            Html(html_content)
        },
        None => Html("<h1>Post not found</h1>".to_string()),
    }
} 