//! Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
//!
//! The models classifies the following categories:
//! - Hate - Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste.
//! - Hate/Threatening - Hateful content that also includes violence or serious harm towards the targeted group.
//! - Self-harm - Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
//! - Sexual - Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
//! - Sexual/minors - Sexual content that includes an individual who is under 18 years old.
//! - Violence - Content that promotes or glorifies violence or celebrates the suffering or humiliation of others.
//! - Violence/graphic - Violent content that depicts death, violence, or serious physical injury in extreme graphic detail.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    types::{Moderation, ModerationParam},
    Client, Result,
};

/// Classifies if text violates OpenAI's Content Policy.
///
/// Related OpenAI docs: [Create Moderation](https://beta.openai.com/docs/api-reference/moderations/create).
///
/// ## Example
/// ```no_run
/// use fieri::{Client, moderation::{ModerationParamBuilder, create}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let param = ModerationParamBuilder::new("I want to kill them.")
///         .model("text-moderation-stable")
///         .build()?;
///
///     let resp = create(&client, &param).await?;
///     println!("{:?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client, param: &ModerationParam) -> Result<Moderation> {
    client.create_moderation(param).await
}

impl Client {
    async fn create_moderation(&self, param: &ModerationParam) -> Result<Moderation> {
        self.post::<ModerationParam, Moderation>("moderations", Some(param))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
