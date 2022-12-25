//! Given a prompt and/or an input image, the model will generate a new image.
//!
//! The Images API provides three methods for interacting with images:
//! - Creating images from scratch based on a text prompt
//! - Creating edits of an existing image based on a new text prompt
//! - Creating variations of an existing image

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{ErrorResp, TokenUsage},
    Client, Result,
};

/// The size of the generated images.
///
/// Must be one of 256x256, 512x512, or 1024x1024.
#[derive(Debug)]
pub enum ImageSize {
    S256x256,
    S512x512,
    S1024x1024,
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageSize::S256x256 => write!(f, "256x256"),
            ImageSize::S512x512 => write!(f, "512x512"),
            ImageSize::S1024x1024 => write!(f, "1024x1024"),
        }
    }
}

impl Serialize for ImageSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// The image generations endpoint allows you to create an original image given a text prompt. Generated images can have a size of 256x256, 512x512, or 1024x1024 pixels.
//
// Smaller sizes are faster to generate.
/// Parameters for [`Generate Image`](generate) request.
#[derive(Debug, Serialize)]
pub struct GenerateImage {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: u8,

    /// The size of the generated images.
    pub size: ImageSize,

    /// The format in which the generated images are returned.
    ///
    /// Must be one of `url or `b64_json`.
    pub response_format: String,

    /// A unique identifier representing your end-user.
    pub user: String,
}

impl Default for GenerateImage {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            n: 1,
            size: ImageSize::S1024x1024,
            response_format: String::from("url"),
            user: String::new(),
        }
    }
}

impl GenerateImage {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            ..Default::default()
        }
    }

    pub fn prompt<T: Into<String>>(mut self, prompt: T) -> Self {
        self.prompt = prompt.into();

        self
    }

    pub fn n(mut self, n: u8) -> Self {
        self.n = n;

        self
    }

    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = size;

        self
    }

    pub fn response_format<T: Into<String>>(mut self, response_format: T) -> Self {
        self.response_format = response_format.into();

        self
    }

    pub fn user<T: Into<String>>(mut self, user: T) -> Self {
        self.user = user.into();

        self
    }
}

/// Response from [Generate](generate), [Edit](edit) & [Variation](variation) requests.
#[derive(Debug, Deserialize, Getters)]
pub struct Image {
    created: Option<u64>,
    data: Option<Links>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// A link to an image.
#[derive(Debug, Deserialize, Getters)]
pub struct Link {
    url: String,
}

type Links = Vec<Link>;

/// Parameters for [`Edit Image`](edit) request.
// TODO
#[allow(dead_code)]
struct EditImage {}

/// Parameters for [`Variate Image`](variate) request.
// TODO
#[allow(dead_code)]
struct VariateImage {}

/// Generate an image from a prompt.
///
/// Related OpenAI docs: [Create Image](https://beta.openai.com/docs/api-reference/images/create)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use openai_rs::{
///     Config, Client,
///     image::{Image, GenerateImage, generate},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///    let param = GenerateImage::new("Dogs playing poker.")
///        .size(ImageSize::S256x256))
///        .n(1);
///
///    let resp = generate(client, &param).await?;
///    println!("{:#?}", resp);
///
///    Ok(())
/// }
/// ```
pub async fn generate(client: Client<'_>, param: &GenerateImage) -> Result<Image> {
    client.generate_image(param).await
}

/// Creates an edited or extended image given an original image and a prompt.
///
/// ## Example
/// ```no_run
///
/// ```
// TODO
pub async fn edit(client: Client<'_>, param: &GenerateImage) -> Result<Image> {
    client.edit_image(param).await
}

/// Creates a variation of an existing image.
///
/// ## Example
/// ```no_run
///
/// ```
// TODO
pub async fn variation(client: Client<'_>, param: &GenerateImage) -> Result<Image> {
    client.variation_image(param).await
}

impl<'a> Client<'a> {
    async fn generate_image(&self, param: &GenerateImage) -> Result<Image> {
        let resp = self
            .post::<&str, GenerateImage, Image>("/images/generations", Some(param))
            .await?;

        Ok(resp)
    }

    async fn edit_image(&self, param: &GenerateImage) -> Result<Image> {
        let resp = self
            .post::<&str, GenerateImage, Image>("/images/edits", Some(param))
            .await?;

        Ok(resp)
    }

    async fn variation_image(&self, param: &GenerateImage) -> Result<Image> {
        let resp = self
            .post::<&str, GenerateImage, Image>("/images/variations", Some(param))
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_generate_image() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let param = GenerateImage::new(String::from("Generate an image reflecting the year 1939."))
            .size(ImageSize::S256x256)
            .n(1);

        let resp = generate(client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }
}
