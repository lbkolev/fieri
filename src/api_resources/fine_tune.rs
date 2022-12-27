//! Manage fine-tuning jobs to tailor a model to your specific training data.
//!
//! Fine-tuning lets you get more out of the models available through the API by providing:
//! - Higher quality results than prompt design.
//! - Ability to train on more examples than can fit in a prompt.
//! - Token savings due to shorter prompts.
//! - Lower latency requests.
//!
//! GPT-3 has been pre-trained on a vast amount of text from the open internet.
//! When given a prompt with just a few examples, it can often intuit what task you are trying to perform and generate a plausible completion.
//! This is often called "few-shot learning."
//!
//! Fine-tuning improves on few-shot learning by training on many more examples than can fit in the prompt,
//! letting you achieve better results on a wide number of tasks.
//! Once a model has been fine-tuned, you won't need to provide examples in the prompt anymore.
//! This saves costs and enables lower-latency requests.

#![allow(unused_imports)]

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{Delete, ErrorResp, TokenUsage},
    Client, Result,
};

/// Delete a fine-tuned model. You must have the Owner role in your organization.
///
/// Related OpenAI docs: [Delete Fine-tuned model](https://beta.openai.com/docs/api-reference/fine-tunes/delete-model)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, fine_tune::delete};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let resp = delete(&client, "model-to-delete").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn delete<T: Into<String> + std::fmt::Display>(
    client: &Client,
    model: T,
) -> Result<Delete> {
    client.delete_fine_tune(model).await
}

impl Client {
    async fn delete_fine_tune<T: Into<String> + std::fmt::Display>(
        &self,
        model: T,
    ) -> Result<Delete> {
        self.delete::<(), Delete>(format!("models/{model}").as_str(), None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[ignore = "requires file deletion"]
    #[tokio::test]
    async fn test_delete_fine_tune() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = delete(&client, "model-to-delete").await?;
        println!("{:#?}", resp);

        assert!(resp.deleted().is_some());
        assert!(resp.error().is_some());
        Ok(())
    }
}
