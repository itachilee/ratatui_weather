use crate::models::{ChatRequest, DeepSeekRequest, DeepSeekResponse, Message};
use anyhow::Result;

pub struct DeepSeekService {
    api_key: String,
    client: reqwest::Client,
}

impl DeepSeekService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn chat(&self, messages: Vec<Message>) -> Result<Message> {
        let request = DeepSeekRequest {
            model: "deepseek-chat".to_string(),
            messages,
            temperature: 0.7,
            max_tokens: 2000,
            stream: false,
        };

        let response = self
            .client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<DeepSeekResponse>()
            .await?;

        response
            .choices
            .first()
            .map(|choice| choice.message.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from API"))
    }
}
