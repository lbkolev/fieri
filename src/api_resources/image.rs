//! Given a prompt and/or an input image, the model will generate a new image.
//!
//! The Images API provides three methods for interacting with images:
//! - Creating images from scratch based on a text prompt
//! - Creating edits of an existing image based on a new text prompt
//! - Creating variations of an existing image

use derive_getters::Getters;
use reqwest::{
    get,
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fs,
    io::{copy, Cursor},
    path::Path,
};

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

/// Parameters for [`Generate Image`](generate) request.
#[derive(Debug, Serialize)]
pub struct GenerateImageParam {
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

impl Default for GenerateImageParam {
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

impl GenerateImageParam {
    pub fn new<T: Into<String>>(prompt: T) -> Self {
        Self {
            prompt: prompt.into(),
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

impl Image {
    /// Save the image(s) to the given directory.
    /// The images will be saved as based on the generated image id.
    ///
    /// For example, a generated image with url `https://oaidalleapiprodscus.blob.core.windows.net/private/org-123/user-456/img-789.png`
    /// Will be saved with a name of `img-789.png` in the given directory.
    /// `
    ///
    /// ## Example
    /// ```rust
    /// // Generate an image based on a prompt and save it locally.
    /// use std::env;
    /// use fieri::{
    ///     Client,
    ///     image::{ImageSize, GenerateImageParam, generate},
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new(env::var("OPENAI_API_KEY")?);
    ///
    ///     let param = GenerateImageParam::new("A cat")
    ///         .size(ImageSize::S256x256)
    ///         .n(1);
    ///
    ///     let image = generate(&client, &param).await?
    ///         .save("/tmp/").await?;
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
#[allow(dead_code)]
#[derive(Debug)]
pub struct EditImageParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    image: P,
    prompt: String,
    mask: String,
    n: u8,
    size: ImageSize,
    response_format: String,
    user: String,
}

impl<P> EditImageParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    pub fn new(image: P, prompt: String) -> Self {
        Self {
            image,
            prompt,
            mask: String::new(),
            n: 1,
            size: ImageSize::S1024x1024,
            response_format: String::from("url"),
            user: String::new(),
        }
    }
}

impl<P> EditImageParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    pub fn mask<T: Into<String>>(mut self, mask: T) -> Self {
        self.mask = mask.into();

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

/// Parameters for [`Variate Image`](variate) request.
#[derive(Debug)]
pub struct VariateImageParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    image: P,
    n: u8,
    size: ImageSize,
    response_format: String,
    user: String,
}

impl<P> VariateImageParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    pub fn new(image: P) -> Self {
        Self {
            image,
            n: 1,
            size: ImageSize::S1024x1024,
            response_format: String::from("url"),
            user: String::new(),
        }
    }
}

impl<P> VariateImageParam<P>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
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
/// use fieri::{
///     Client,
///     image::{ImageSize, GenerateImageParam, generate},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = GenerateImageParam::new("Dogs playing poker.")
///        .size(ImageSize::S256x256)
///        .n(1);
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
/// use fieri::{Client, image::{ImageSize, EditImageParam, edit}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = EditImageParam::new("./payloads/image_tests.png", String::from("A dog playing poker."))
///        .size(ImageSize::S256x256);
///
///     let resp = edit(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn edit<P>(client: &Client, param: &EditImageParam<P>) -> Result<Image>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    client.edit_image(param).await
}

/// Creates a variation of an existing image.
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, image::{ImageSize, VariateImageParam, variation}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = VariateImageParam::new("./payloads/image_tests.png")
///       .size(ImageSize::S512x512);
///
///     let resp = variation(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
// TODO: refactor
pub async fn variation<P>(client: &Client, param: &VariateImageParam<P>) -> Result<Image>
where
    P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
{
    client.variation_image(param).await
}

impl Client {
    async fn generate_image(&self, param: &GenerateImageParam) -> Result<Image> {
        let resp = self
            .post::<GenerateImageParam, Image>("images/generations", Some(param))
            .await?;

        Ok(resp)
    }

    // TODO: refactor
    async fn edit_image<P>(&self, param: &EditImageParam<P>) -> Result<Image>
    where
        P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
    {
        let data = fs::read(param.image)?;
        let part = Part::bytes(data).file_name(param.image);
        let form = Form::new()
            .part("image", part)
            .text("prompt", param.prompt.clone())
            .text("mask", param.mask.clone())
            .text("n", param.n.to_string())
            .text("size", param.size.to_string())
            .text("response_format", param.response_format.clone())
            .text("user", param.user.clone());

        let resp = self
            .post_data::<EditImageParam<P>, Image>("images/edits", form)
            .await?;

        Ok(resp)
    }

    // TODO: refactor
    async fn variation_image<P>(&self, param: &VariateImageParam<P>) -> Result<Image>
    where
        P: AsRef<Path> + Into<Cow<'static, str>> + Copy,
    {
        let data = fs::read(param.image)?;
        let part = Part::bytes(data).file_name(param.image);
        let form = Form::new()
            .part("image", part)
            .text("n", param.n.to_string())
            .text("size", param.size.to_string())
            .text("response_format", param.response_format.clone())
            .text("user", param.user.clone());
        let resp = self
            .post_data::<VariateImageParam<P>, Image>("images/variations", form)
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_generate_image() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param =
            GenerateImageParam::new(String::from("Generate an image reflecting the year 1939."))
                .size(ImageSize::S256x256)
                .n(1);
        let resp = generate(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());

        let _ = resp.save("/tmp").await?;
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_edit_image() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = EditImageParam::new(
            "./payloads/image_tests.png",
            String::from("Generate an image reflecting the year 1939."),
        )
        .size(ImageSize::S256x256)
        .n(2);

        let resp = edit(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_variation_image() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = VariateImageParam::new("./payloads/image_tests.png")
            .size(ImageSize::S256x256)
            .n(2);

        let resp = variation(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }
}
