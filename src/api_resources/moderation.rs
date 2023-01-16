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

use crate::{api_resources::TokenUsage, Client, Result};

/// Parameters for [`Create Moderation`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct ModerationParam {
    /// The content moderations model to use for the request.
    model: Option<String>,

    /// The input text to classify.
    input: String,
}

impl ModerationParamBuilder {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: Some(input.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create Moderation`](create) request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Moderation {
    pub id: String,
    pub model: String,
    pub flagged: bool,
    pub results: Vec<ModerationResult>,

    pub token_usage: Option<TokenUsage>,
}

/// The result of the content moderation request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModerationResult {
    pub categories: Categories,
    pub category_scores: CategoryScores,
}

/// Contains a per-category binary content policy violation flags.
///
/// For each category, the value is `true` if the model flags the corresponding category as violated, `false` otherwise.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Categories {
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

/// Contains a per-category raw scores output by the model, denoting the model's confidence that the input violates the OpenAI's policy for the category.
///
/// The value is between 0 and 1, where higher values denote higher confidence.
///
/// The scores should not be interpreted as probabilities.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CategoryScores {
    pub hate: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    pub violence: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}

/// Classifies if text violates OpenAI's Content Policy.
///
/// Related OpenAI docs: [Create Moderation](https://beta.openai.com/docs/api-reference/moderations/create).
///
/// ## Example
/// ```rust
/// use std::env;
/// use fieri::{Client, moderation::{ModerationParamBuilder, create}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
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
    use std::env;

    #[tokio::test]
    async fn test_create_moderation() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = ModerationParamBuilder::new("That shouldn't be flagged as flagged, even though it posseses KILL, MURDER and SUICIDE")
            .model("text-moderation-stable")
            .build()?;

        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(!resp.flagged);
        assert!(resp.token_usage.is_none());
        Ok(())
    }
}
