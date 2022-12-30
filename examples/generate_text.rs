//! Generate text using the OpenAI API

use fieri::{
    completion::{create, CompletionParamBuilder},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = CompletionParamBuilder::new("ada")
        .prompt("Generate a plot for an absurd interstellar parody.")
        .max_tokens(500)
        .temperature(0.9)
        .top_p(1.0)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .build()?;

    let resp = create(&client, &param).await?;

    if resp.error().is_none() {
        println!("Generated text: {}", resp.choices().first().unwrap().text());
    }
    Ok(())
}
