use derive_getters::Getters;
use serde::Deserialize;

use crate::{client::Client, Result};

#[derive(Debug, Deserialize, Getters)]
pub struct ModelsResponse {
    data: Vec<ModelResponse>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct ModelResponse {
    id: String,
    object: String,
    created: u64,
    owned_by: String,
    permission: Vec<Permissions>,
    root: String,
    parent: Option<String>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct Permissions {
    id: String,
    object: String,
    created: u64,
    allow_create_engine: bool,
    allow_sampling: bool,
    allow_logprobs: bool,
    allow_search_indices: bool,
    allow_view: bool,
    allow_fine_tuning: bool,
    organization: String,
    group: Option<String>,
    is_blocking: bool,
}

/// Object containing the available Models that are offered for usage through the API.
pub enum Model {
    Ada,
    Babbage,
    Curie,
    Davinci,
    TextAda001,
    TextBabbage001,
    TextCurie001,
    TextDavinci001,
    TextDavinci002,
    TextDavinci003,
    CodeCushman001,
    CodeDavinci003,
    CurieInstructBeta,
    DavinciInstructBeta,
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Model::*;

        match &self {
            Ada => write!(f, "ada"),
            Babbage => write!(f, "babbage"),
            Curie => write!(f, "curie"),
            Davinci => write!(f, "davinci"),
            TextAda001 => write!(f, "text-ada-001"),
            TextBabbage001 => write!(f, "text-babbage-001"),
            TextCurie001 => write!(f, "text-curie-001"),
            TextDavinci001 => write!(f, "text-davinci-001"),
            TextDavinci002 => write!(f, "text-davinci-002"),
            TextDavinci003 => write!(f, "text-davinci-003"),
            CodeCushman001 => write!(f, "code-cushman-001"),
            CodeDavinci003 => write!(f, "code-davinci-003"),
            CurieInstructBeta => write!(f, "curie-instruct-beta"),
            DavinciInstructBeta => write!(f, "davinci-instruct-beta"),
        }
    }
}

/// Lists the currently available models, and provides basic information about each one.
pub async fn list_models(client: &Client<'_>) -> Result<ModelsResponse> {
    client.list_models().await
}

/// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
pub async fn retrieve_model(client: &Client<'_>, model: Model) -> Result<ModelResponse> {
    client.retrieve_model(model).await
}

impl<'a> Client<'a> {
    async fn list_models(&self) -> Result<ModelsResponse> {
        let resp = self
            .get::<(), ModelsResponse>("/models".to_string(), None)
            .await?;

        Ok(resp)
    }

    async fn retrieve_model(&self, model: Model) -> Result<ModelResponse> {
        let resp = self
            .get::<(), ModelResponse>(format!("/models/{}", model), None)
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Client, config::Config};
    use more_asserts as ma;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list_models() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let resp = retrieve_model(&client, Model::TextBabbage001).await?;

        assert_eq!(resp.root(), "text-babbage-001");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_model() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let resp = list_models(&client).await?;

        ma::assert_gt!(resp.data().len(), 1);
        Ok(())
    }
}
