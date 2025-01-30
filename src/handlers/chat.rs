use crate::models::{ApiResponse, AppState, ChatRequest, ChatResponse, Message};
use crate::services::deepseek::DeepSeekService;
use actix_web::{post, web, HttpResponse};

#[post("/chat")]
pub async fn chat(req: web::Json<ChatRequest>, data: web::Data<AppState>) -> HttpResponse {
    let service = DeepSeekService::new(data.deepseek_api_key.clone());

    let messages = vec![Message {
        role: "user".to_string(),
        content: req.message.clone(),
    }];

    match service.chat(messages).await {
        Ok(response) => {
            let chat_response = ChatResponse {
                conversation_id: uuid::Uuid::new_v4().to_string(),
                response: response.content,
                history: vec![],
            };
            HttpResponse::Ok().json(ApiResponse::success(chat_response))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}
