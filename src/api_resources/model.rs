//! List and describe the various models available in the API.

use serde::{Deserialize, Serialize};

use crate::{
    types::{Model, Models},
    Client, Result,
};

/// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
///
/// Related OpenAI docs: [Retrieve a Model](https://beta.openai.com/docs/api-reference/models/retrieve)
///
/// ## Example:
/// ```no_run
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

}
