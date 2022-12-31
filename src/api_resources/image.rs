//! Given a prompt and/or an input image, the model will generate a new image.
//!
//! The Images API provides three methods for interacting with images:
//! - Creating images from scratch based on a text prompt
//! - Creating edits of an existing image based on a new text prompt
//! - Creating variations of an existing image

use derive_builder::Builder;
use derive_getters::Getters;
use reqwest::{
    get,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{
    borrow::Cow,
    fs,
    io::{copy, Cursor},
    path::Path,
};

use crate::{api_resources::TokenUsage, Client, Result};

/// The size of the generated images.
///
/// Must be one of 256x256, 512x512, or 1024x1024.
#[derive(Clone, Debug, Default)]
pub enum ImageSize {
    S256x256,
    S512x512,
    #[default]
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

/// Parameters for [`Generate Image`](generate) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct GenerateImageParam {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    n: Option<u8>,

    /// The size of the generated images.
    size: Option<ImageSize>,

    /// A unique identifier representing your end-user.
    user: Option<String>,
}

impl GenerateImageParamBuilder {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: Some(prompt.into()),
            ..Default::default()
        }
    }
}

/// Response from [Generate](generate), [Edit](edit) & [Variation](variate) requests.
#[derive(Debug, Deserialize, Getters)]
pub struct Image {
    created: Option<u64>,
    data: Option<Links>,
    token_usage: Option<TokenUsage>,
}

impl Image {
    /// Save the image(s) to the given directory.
    /// The images will be saved as based on the generated image id.
    ///
    /// For example, a generated image with url `https://oaidalleapiprodscus.blob.core.windows.net/private/org-123/user-456/img-789.png`
    /// Will be saved with a name of `img-789.png` in the given directory.
    ///
    ///
    /// ## Example
    /// ```rust
    /// // Generate an image based on a prompt and save it locally.
    /// use std::env;
    /// use fieri::{Client, image::{ImageSize, GenerateImageParamBuilder, generate}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new(env::var("OPENAI_API_KEY")?);
    ///
    ///     let param = GenerateImageParamBuilder::new("A cat")
    ///         .size(ImageSize::S256x256)
    ///         .n(1)
    ///         .build()?;
    ///
    ///     let image = generate(&client, &param)
    ///         .await?
    ///         .save("/tmp/")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub async fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let Some(data) = &self.data {
            for (i, link) in data.iter().enumerate() {
                let resp = get(&link.url).await?;

                let def_img_name = format!("image_{i}.png");
                let fname = resp
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .unwrap_or(def_img_name.as_str());

                let full_path = Path::new(path.as_ref()).join(fname);
                let mut file = fs::File::create(full_path)?;
                let mut content = Cursor::new(resp.bytes().await?);
                copy(&mut content, &mut file)?;
            }
        }

        Ok(())
    }
}

/// link to an image.
#[derive(Debug, Deserialize, Getters)]
pub struct Link {
    url: String,
}

type Links = Vec<Link>;

/// Parameters for [`Edit Image`](edit) request.
#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct EditImageParam {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    n: u8,

    /// The size of the generated images.
    size: ImageSize,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    user: String,
}

impl Default for EditImageParam {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            n: 1,
            size: ImageSize::S1024x1024,
            user: String::new(),
        }
    }
}

impl EditImageParamBuilder {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: Some(prompt.into()),
            ..Default::default()
        }
    }
}

/// Parameters for [`Variate Image`](variate) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct VariateImageParam {
    /// The number of images to generate. Must be between 1 and 10.
    n: u8,

    /// The size of the generated images.
    size: ImageSize,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    user: String,
}

impl Default for VariateImageParam {
    fn default() -> Self {
        Self {
            n: 1,
            size: ImageSize::S1024x1024,
            user: String::new(),
        }
    }
}

impl VariateImageParamBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Generate an image from a prompt.
/// The image generations endpoint allows you to create an original image given a text prompt. Generated images can have a size of `256x256`, `512x512`, or `1024x1024` pixels.
///
/// Smaller sizes are faster to generate.
///
/// Related OpenAI docs: [Create Image](https://beta.openai.com/docs/api-reference/images/create)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, image::{ImageSize, GenerateImageParamBuilder, generate}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = GenerateImageParamBuilder::new("Dogs playing poker.")
///         .size(ImageSize::S256x256)
///         .n(1)
///         .build()?;
///
///     let resp = generate(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn generate(client: &Client, param: &GenerateImageParam) -> Result<Image> {
    client.generate_image(param).await
}

/// Creates an edited or extended image given an original image and a prompt.
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, image::{ImageSize, EditImageParamBuilder, edit}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = EditImageParamBuilder::new("Transform the image to a dog playing poker.")
///         .size(ImageSize::S256x256)
///         .n(1)
///         .build()?;
///
///     let resp = edit(&client, "path-to-image", &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn edit<P>(client: &Client, image: P, param: &EditImageParam) -> Result<Image>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    client.edit_image(image, param).await
}

/// Creates a variation of a given image.
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, image::{ImageSize, VariateImageParamBuilder, variate}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = VariateImageParamBuilder::new()
///         .size(ImageSize::S256x256)
///         .build()?;
///
///     let resp = variate(&client, "path-to-image", &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn variate<P>(client: &Client, image: P, param: &VariateImageParam) -> Result<Image>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    client.variate_image(image, param).await
}

impl Client {
    async fn generate_image(&self, param: &GenerateImageParam) -> Result<Image> {
        self.post::<GenerateImageParam, Image>("images/generations", Some(param))
            .await
    }

    async fn edit_image<P>(&self, image: P, param: &EditImageParam) -> Result<Image>
    where
        P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
    {
        let data = fs::read(image)?;
        let part = Part::bytes(data).file_name(image);
        let form = Form::new()
            .part("image", part)
            .text("prompt", "22")
            .text("n", param.n.to_string())
            .text("size", param.size.to_string())
            .text("user", param.user.to_string());

        self.post_data::<Image>("images/edits", form).await
    }

    async fn variate_image<P>(&self, image: P, param: &VariateImageParam) -> Result<Image>
    where
        P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
    {
        let data = fs::read(image)?;
        let part = Part::bytes(data).file_name(image);
        let form = Form::new()
            .part("image", part)
            .text("n", param.n.to_string())
            .text("size", param.size.to_string())
            .text("user", param.user.to_string());

        self.post_data::<Image>("images/variations", form).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[ignore = "expensive"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_generate_image() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = GenerateImageParamBuilder::new("Generate an image reflecting the year 1939.")
            .size(ImageSize::S256x256)
            .n(1)
            .build()?;

        let resp = generate(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.token_usage().is_none());
        Ok(())
    }

    #[ignore = "expensive"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_edit_image() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = EditImageParamBuilder::new("Make it more generic")
            .size(ImageSize::S256x256)
            .n(1)
            .build()?;

        let resp = edit(&client, "assets/image_tests.png", &param).await?;
        println!("{:#?}", resp);

        assert!(resp.token_usage().is_none());
        Ok(())
    }

    #[ignore = "expensive"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_variate_image() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = VariateImageParamBuilder::new()
            .size(ImageSize::S256x256)
            .n(1)
            .build()?;

        let resp = variate(&client, "./assets/image_tests.png", &param).await?;
        println!("{:#?}", resp);

        assert!(resp.token_usage().is_none());
        Ok(())
    }
}
