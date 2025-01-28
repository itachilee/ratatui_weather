use ratatui_weather::api;
// #[tokio::main]
#[actix_web::main]

pub async fn main() -> std::io::Result<()> {
    if let Err(e) = api::run().await {
        println!("Error: {}", e);
    }
    Ok(())
}
