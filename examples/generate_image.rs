//! Generate 2 images with size 512x512 using the OpenAI API's DALL-E model

use fieri::{
    image::{generate, GenerateImageParamBuilder, ImageSize},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = GenerateImageParamBuilder::new("Generate a simple landscape of the most beautiful part of Mars, that can preferably be used as a logo for an unrelated project.")
       .size(ImageSize::S512x512)
       .n(2)
       .build()?;

    let resp = generate(&client, &param).await?;
    println!("{:#?}", resp);

    Ok(())
}
