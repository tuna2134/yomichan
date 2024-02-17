use twilight_http::Client as HttpClient;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let http = HttpClient::new(env::var("DISCORD_TOKEN")?);
    println!("Hello, world!");
    Ok(())
}
