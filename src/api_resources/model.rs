//! List and describe the various models available in the API.

use serde::{Deserialize, Serialize};

use crate::{api_resources::TokenUsage, Client, Result};

/// Response from [List Models](list) request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Models {
    pub data: Vec<Model>,

    pub token_usage: Option<TokenUsage>,
}

/// Response from [Retrieve a Model](retrieve) request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub permission: Vec<Permissions>,
    pub root: String,
    pub parent: Option<String>,

    pub token_usage: Option<TokenUsage>,
}

/// Types of permissions that can be applied to a model.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Permissions {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
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
///     let resp = retrieve(&client, "text-babbage-001").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn retrieve(client: &Client, model: impl Into<String>) -> Result<Model> {
    client.retrieve(model.into()).await
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
    async fn retrieve(&self, model: String) -> Result<Model> {
        self.get::<(), Model>(&format!("models/{model}"), None)
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
    async fn test_model_list() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = retrieve(&client, "text-babbage-001").await?;
        println!("{:#?}", resp);

        assert_eq!(resp.root, "text-babbage-001");
        assert!(resp.token_usage.is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_model_retrieve() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = list(&client).await?;
        println!("{:#?}", resp);

        assert!(!resp.data.is_empty());
        assert!(resp.token_usage.is_none());
        Ok(())
    }
}
