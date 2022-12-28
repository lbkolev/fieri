//! Generate 3 images using the OpenAI API's DALL-E model

use fieri::{
    image::{generate, GenerateImageParam, ImageSize},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = GenerateImageParam::new(
        "A bunch of cats dancing tango on the top of the highest mountain on Mars.",
    )
    .size(ImageSize::S1024x1024)
    .n(1);

    let image = generate(&client, &param).await?.save("/tmp/").await?;

    Ok(())
}
