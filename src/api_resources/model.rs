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
/// ```no_run
/// use std::env;
/// use fieri::{Client, model::retrieve};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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
/// ```no_run
/// use std::env;
/// use fieri::{Client, model::list};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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

    #[test]
    fn test_model_list() {
        let resp: Models = serde_json::from_str(
            r#"
            {
                "data": [
                  {
                    "id": "model-id-0",
                    "object": "model",
                    "created": 1623155849,
                    "owned_by": "organization-owner",
                    "permission": [],
                    "root": "model-id-1"
                  },
                  {
                    "id": "model-id-1",
                    "object": "model",
                    "created": 11,
                    "owned_by": "organization-owner",
                    "permission": [],
                    "root": "model-id-0"
                  },
                  {
                    "id": "model-id-2",
                    "object": "model",
                    "created": 1234567890,
                    "owned_by": "openai",
                    "permission": [],
                    "root": "model-id-2"
                  }
                ],
                "object": "list"
              }              
            "#,
        )
        .unwrap();

        assert_eq!(resp.data.len(), 3);
        assert_eq!(resp.data[0].id, "model-id-0");
        assert!(resp.token_usage.is_none());
    }

    #[test]
    fn test_model_retrieve() {
        let resp: Model = serde_json::from_str(
            r#"
            {
                "id": "text-davinci-003",
                "object": "model",
                "created": 1623155849,
                "owned_by": "openai",
                "permission": [],
                "root": "text-davinci-003"
              }              
            "#,
        )
        .unwrap();

        assert_eq!(resp.id, "text-davinci-003");
        assert!(resp.token_usage.is_none());
    }
}
