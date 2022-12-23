use std::env;

use openai_rs::api_resources::model::retrieve;
use openai_rs::config::Model;
use openai_rs::{client::Client, config::Config};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(env::var("OPENAI_API_KEY")?);
    let client = Client::new(&config);

    let resp = retrieve(&client, Model::Davinci).await?;
    println!("Resp1 {:#?}", resp);
    Ok(())
}
