//! List and describe the various models available in the API.
//!
//! You can refer to the Models documentation to understand what models are available and the differences between them.

use derive_getters::Getters;
use serde::Deserialize;

use crate::{client::Client, config::Model, Result};

#[derive(Debug, Getters, Deserialize)]
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

#[derive(Debug, Getters, Deserialize)]
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

/// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
///
/// Related OpenAI docs: [Retrieve a Model](https://beta.openai.com/docs/api-reference/models/retrieve)
///
/// ## Example:
/// ```rust
/// use std::env;
/// use openai_rs::{
///     config::{Config, Model},
///     client::Client,
///     api_resources::model::retrieve
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let resp = retrieve(&client, Model::TextBabbage001).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn retrieve(client: &Client<'_>, model: Model) -> Result<ModelResponse> {
    client.retrieve(model).await
}

/// Lists the currently available models, and provides basic information about each one.
///
/// Related OpenAI docs: [List Models](https://beta.openai.com/docs/api-reference/models/list)
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     config::{Config, Model},
///     client::Client,
///     api_resources::model::list
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let resp = list(&client).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn list(client: &Client<'_>) -> Result<ModelsResponse> {
    client.list().await
}

impl<'a> Client<'a> {
    async fn list(&self) -> Result<ModelsResponse> {
        let resp = self
            .get::<(), ModelsResponse>("/models".to_string(), None)
            .await?;

        Ok(resp)
    }

    async fn retrieve(&self, model: Model) -> Result<ModelResponse> {
        let resp = self
            .get::<(), ModelResponse>(format!("/models/{model}"), None)
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use more_asserts as ma;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let resp = retrieve(&client, Model::TextBabbage001).await?;

        assert_eq!(resp.root(), "text-babbage-001");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let resp = list(&client).await?;

        ma::assert_gt!(resp.data().len(), 1);
        Ok(())
    }
}
