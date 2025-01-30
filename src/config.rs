use dotenv::dotenv;
use std::env;

pub fn init_config() -> String {
    dotenv().ok();
    env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set")
}
