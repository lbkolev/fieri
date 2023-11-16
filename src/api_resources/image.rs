//! Given a prompt and/or an input image, the model will generate a new image.
//!
//! The Images API provides three methods for interacting with images:
//! - Creating images from scratch based on a text prompt
//! - Creating edits of an existing image based on a new text prompt
//! - Creating variations of an existing image

use derive_builder::Builder;
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
    str::FromStr,
};

use crate::{
    types::{EditImageParam, GenerateImageParam, Image, VariateImageParam},
    Client, Result,
};

/// The image generations endpoint allows you to create an original image given a text prompt. Generated images can have a size of `256x256`, `512x512`, or `1024x1024` pixels.
///
/// Smaller sizes are faster to generate.
///
/// Related OpenAI docs: [Create Image](https://beta.openai.com/docs/api-reference/images/create)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, image::{ImageSize, GenerateImageParamBuilder, generate}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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
/// use fieri::{Client, image::{ImageSize, EditImageParamBuilder, edit}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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
/// use fieri::{Client, image::{ImageSize, VariateImageParamBuilder, variate}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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
}
