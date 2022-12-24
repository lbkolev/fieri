//! Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
///!
///! The models classifies the following categories:
///! - Hate - Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste.
///! - Hate/Threatening - Hateful content that also includes violence or serious harm towards the targeted group.
///! - Self-harm - Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
///! - Sexual - Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
///! - Sexual/minors - Sexual content that includes an individual who is under 18 years old.
///! - Violence - Content that promotes or glorifies violence or celebrates the suffering or humiliation of others.
///! - Violence/graphic - Violent content that depicts death, violence, or serious physical injury in extreme graphic detail.
///!
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{api_resources::ErrorResp, Client, Models, Result};

/// Parameters for [`create`](crate::api_resources::moderation::create) moderation request.
#[derive(Debug, Clone, Serialize)]
pub struct ModerationParam {
    /// The content moderations model to use for the request.
    pub model: Option<Models>,

    /// The input text to classify.
    pub input: String,
}

impl Default for ModerationParam {
    fn default() -> Self {
        Self {
            model: Some(Models::TextModerationLatest),
            input: String::new(),
        }
    }
}

impl ModerationParam {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: Option<Models>) -> Self {
        self.model = model;

        self
    }

    pub fn input(mut self, input: String) -> Self {
        self.input = input;

        self
    }
}

/// Response from [`create`](crate::api_resources::moderation::create) moderation request.
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct ModerationResp {
    id: Option<String>,
    model: Option<String>,
    results: Option<Vec<ModerationResult>>,
    error: Option<ErrorResp>,
    flagged: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Getters)]
pub struct ModerationResult {
    categories: Categories,
    category_scores: CategoryScores,
}

#[derive(Debug, Clone, Deserialize, Getters)]
pub struct Categories {
    hate: bool,
    #[serde(rename = "hate/threatening")]
    hate_threatening: bool,
    #[serde(rename = "self-harm")]
    self_harm: bool,
    sexual: bool,
    #[serde(rename = "sexual/minors")]
    sexual_minors: bool,
    violence: bool,
    #[serde(rename = "violence/graphic")]
    violence_graphic: bool,
}

#[derive(Debug, Clone, Deserialize, Getters)]
pub struct CategoryScores {
    hate: f64,
    #[serde(rename = "hate/threatening")]
    hate_threatening: f64,
    #[serde(rename = "self-harm")]
    self_harm: f64,
    sexual: f64,
    #[serde(rename = "sexual/minors")]
    sexual_minors: f64,
    violence: f64,
    #[serde(rename = "violence/graphic")]
    violence_graphic: f64,
}

/// Classifies if text violates OpenAI's Content Policy
///
/// Related OpenAI docs: [Create moderation](https://beta.openai.com/docs/api-reference/moderations/create).
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::moderation::{
///         create,
///         ModerationParam,
///         ModerationResp,
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let param = ModerationParam::new()
///         .input("I want to kill them.".to_string());
///     let resp: ModerationResp = create(&client, &param).await?;
///     println!("{:?}", resp);
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client<'_>, param: &ModerationParam) -> Result<ModerationResp> {
    client.create_moderation(param).await
}

impl<'a> Client<'a> {
    async fn create_moderation(&self, param: &ModerationParam) -> Result<ModerationResp> {
        let resp = self
            .post::<ModerationParam, ModerationResp>("/moderations".to_string(), Some(param))
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::env;

    #[tokio::test]
    async fn test_create_moderation() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let param = ModerationParam::new()
            .model(Some(Models::TextModerationStable))
            .input("That shouldn't be flagged as flagged, even though it posseses KILL, MURDER and SUICIDE.".to_string());

        let resp = create(&client, &param).await?;

        println!("{:#?}", resp);
        assert!(resp.error().is_none());
        assert!(resp.flagged().is_none());
        Ok(())
    }
}
