//! Generate 2 images with size 512x512 using the OpenAI's DALL-E model

use fieri::{
    image::{generate, GenerateImageParamBuilder, ImageSize},
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let param = GenerateImageParamBuilder::new("Generate a simple landscape of the most beautiful part of Mars, that can preferably be used as a logo for an unrelated project.")
       .size(ImageSize::S512x512)
       .n(2)
       .build()?;

    let resp = generate(&client, &param).await?;
    println!("{:#?}", resp);

    Ok(())
}
