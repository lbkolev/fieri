//! List and describe the various models available in the API.

use derive_getters::Getters;
use serde::Deserialize;

use crate::{
    api_resources::{ErrorResp, TokenUsage},
    Client, Result,
};

/// Response from [List Models](list) request.
#[derive(Debug, Deserialize, Getters)]
pub struct Models {
    data: Vec<Model>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Response from [Retrieve a Model](retrieve) request.
#[derive(Debug, Deserialize, Getters)]
pub struct Model {
    id: String,
    object: String,
    created: u64,
    owned_by: String,
    permission: Vec<Permissions>,
    root: String,
    parent: Option<String>,
    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Types of permissions that can be applied to a model.
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

/// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
///
/// Related OpenAI docs: [Retrieve a Model](https://beta.openai.com/docs/api-reference/models/retrieve)
///
/// ## Example:
/// ```rust
/// use std::env;
/// use fieri::{Client, model::retrieve};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///     let resp = retrieve(&client, fieri::Models::TextBabbage001).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn retrieve(client: &Client, model: crate::Models) -> Result<Model> {
    client.retrieve(model).await
}

/// Lists the currently available models, and provides basic information about each one.
///
/// Related OpenAI docs: [List Models](https://beta.openai.com/docs/api-reference/models/list)
///
/// ## Example
/// ```rust
/// use std::env;
/// use fieri::{Client, model::list};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///     let resp = list(&client).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn list(client: &Client) -> Result<Models> {
    client.list().await
}

impl Client {
    async fn retrieve(&self, model: crate::Models) -> Result<Model> {
        self.get::<(), Model>(format!("models/{model}").as_str(), None)
            .await
    }

    async fn list(&self) -> Result<Models> {
        self.get::<(), Models>("models", None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = retrieve(&client, crate::Models::TextBabbage001).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.root(), "text-babbage-001");
        assert!(resp.error().is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = list(&client).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }
}
