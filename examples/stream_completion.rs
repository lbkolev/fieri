//! Create a completion stream for the provided prompt and parameters.

use fieri::{
    completion::{create_with_stream, CompletionParamBuilder},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = CompletionParamBuilder::new("ada")
        .prompt("unnecessarily lo")
        .temperature(0.5)
        .build()?;

    let mut resp = create_with_stream(&client, &param).await?;

    while let Some(chunk) = resp.chunk().await? {
        let val = String::from_utf8(chunk.to_vec())?;
        println!("{}", val);
    }

    Ok(())
}
