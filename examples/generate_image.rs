use openai_rs::{
    image::{generate, GenerateImage, ImageSize},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY")?);

    let param = GenerateImage::new("Generate a simple landscape of the most beautiful part of Mars, that can preferably be used as a logo for an unrelated project.")
       .size(ImageSize::S1024x1024)
       .n(3);
    let resp = generate(client, &param).await?;
    println!("{:#?}", resp);

    Ok(())
}
