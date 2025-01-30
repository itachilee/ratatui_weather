use actix_web::{
    get, http::StatusCode, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
    ResponseError,
};
use serde::{Deserialize, Serialize};
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
            .service(chat)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    std::env::var("API_KEY").expect("DEEPSEEK_API_KEY not set")
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

#[derive(Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

// 客户端请求结构
#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

// 服务端响应结构
#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

// 处理聊天请求
#[post("/chat")]
async fn chat(req: web::Json<ChatRequest>) -> HttpResponse {
    let api_key = get_api_key();

    let client = reqwest::Client::new();

    // 构建请求消息
    let messages = vec![Message {
        role: "user".to_string(),
        content: req.message.clone(),
    }];

    // 构建 DeepSeek API 请求
    let deepseek_req = DeepSeekRequest {
        model: "deepseek-chat".to_string(),
        messages,
        temperature: 0.7,
        max_tokens: 2000,
        stream: false,
    };

    // 发送请求到 DeepSeek API
    let response = client
        .post("https://api.deepseek.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&deepseek_req)
        .send()
        .await;

    match response {
        Ok(res) => {
            match res.json::<DeepSeekResponse>().await {
                Ok(data) => {
                    // 从响应中提取 AI 的回复
                    if let Some(choice) = data.choices.first() {
                        HttpResponse::Ok().json(ChatResponse {
                            response: choice.message.content.clone(),
                        })
                    } else {
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "没有收到有效的回复"
                        }))
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("解析响应失败: {}", e)
                })),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("请求失败: {}", e)
        })),
    }
}
