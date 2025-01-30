use actix_web::{post, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, sync::Mutex};

// 添加新的类型定义
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Conversation {
    messages: Vec<Message>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

// AppState 用于存储对话历史
struct AppState {
    conversations: Mutex<HashMap<String, Conversation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
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

// 修改请求结构以支持会话 ID
#[derive(Deserialize)]
struct ChatRequest {
    conversation_id: Option<String>,
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    conversation_id: String,
    response: String,
    history: Vec<Message>,
}

// 生成唯一的会话 ID
fn generate_conversation_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

// 处理带历史记录的聊天请求
#[post("/chat-with-history")]
async fn chat_with_history(req: web::Json<ChatRequest>, data: web::Data<AppState>) -> HttpResponse {
    let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY not set");
    let client = reqwest::Client::new();

    // 获取或创建会话
    let conversation_id = req
        .conversation_id
        .clone()
        .unwrap_or_else(generate_conversation_id);

    let mut conversations = data.conversations.lock().unwrap();

    // 获取或创建新的对话历史
    let conversation = conversations
        .entry(conversation_id.clone())
        .or_insert_with(|| Conversation {
            messages: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

    // 添加用户新消息
    let user_message = Message {
        role: "user".to_string(),
        content: req.message.clone(),
    };
    conversation.messages.push(user_message.clone());

    // 构建 DeepSeek API 请求
    let deepseek_req = DeepSeekRequest {
        model: "deepseek-chat".to_string(),
        messages: conversation.messages.clone(),
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
                    if let Some(choice) = data.choices.first() {
                        // 将 AI 响应添加到历史记录
                        conversation.messages.push(choice.message.clone());
                        conversation.updated_at = chrono::Utc::now();

                        HttpResponse::Ok().json(ChatResponse {
                            conversation_id: conversation_id,
                            response: choice.message.content.clone(),
                            history: conversation.messages.clone(),
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

// 获取对话历史的接口
#[post("/conversation/{conversation_id}")]
async fn get_conversation(
    conversation_id: web::Path<String>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let conversations = data.conversations.lock().unwrap();

    if let Some(conversation) = conversations.get(&conversation_id.into_inner()) {
        HttpResponse::Ok().json(conversation)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "未找到该对话"
        }))
    }
}
