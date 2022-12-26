use fieri::{model::retrieve, Client, Models};
use std::env;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let resp = retrieve(&client, Models::Davinci).await?;
    println!("Resp1 {:#?}", resp);
    Ok(())
}
