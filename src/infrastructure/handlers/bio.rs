use axum::response::Html;
use askama::Template;
use chrono::Datelike;

#[derive(Template)]
#[template(path = "bio.html")]
struct BioTemplate {
    year: i32,
    experiences: Vec<Experience>,
    education: Vec<Education>,
}

struct Experience {
    title: String,
    company: String,
    period: String,
    description: String,
}

struct Education {
    school: String,
    degree: String,
    year: String,
    details: String,
}

pub async fn handle_bio() -> Html<String> {
    let experiences = vec![
        Experience {
            title: "Engineering Manager".to_string(),
            company: "Web3 Startup".to_string(),
            period: "2024 - Present".to_string(),
            description: "Leading development teams in building decentralized applications and blockchain solutions.".to_string(),
        },
        Experience {
            title: "Senior Software Engineer".to_string(),
            company: "Blockchain Platform".to_string(),
            period: "2020 - 2024".to_string(),
            description: "Architected and implemented scalable blockchain infrastructure and smart contract systems.".to_string(),
        },
    ];

    let education = vec![
        Education {
            school: "Harvard College".to_string(),
            degree: "AB Physics".to_string(),
            year: "2011".to_string(),
            details: "Thesis on quantum computing applications in cryptography".to_string(),
        },
    ];

    let template = BioTemplate {
        year: chrono::Local::now().year(),
        experiences,
        education,
    };
    Html(template.render().unwrap())
} 