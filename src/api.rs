use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use serde_json::json;
struct AppState {
    api_key: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            api_key: "".to_string(),
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, World!"
}

pub async fn run() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(hello)
            .service(get_user_info)
            .service(completions)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    std::env::var("API_KEY").unwrap()
}

static API_COMPLETIONS: &str = "https://api.deepseek.com/chat/completions";

#[get("/users/{user_id}")]
async fn get_user_info(state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    format!("User ID: {}", user_id)
}

#[get("/completions/{message}")]
async fn completions(message: web::Path<String>) -> impl Responder {
    let client = reqwest::Client::new();
    let response = client
        .post(API_COMPLETIONS)
        .header("Authorization", format!("Bearer {}", get_api_key()))
        .json(&json!({
            "messages": [{"role": "user", "content": message.into_inner()}],
            "model": "deepseek-chat"
        }))
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}
