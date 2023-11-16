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

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

use crate::{
    types::{CreateFineTuneParam, Delete, FineTune, ListEvents, ListFineTune},
    Client, Result,
};

/// Creates a job that fine-tunes a specified model from a given dataset.
///
/// Related OpenAI docs: [Create Fine-tune](https://beta.openai.com/docs/api-reference/fine-tunes/create)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::{create, CreateFineTuneParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///     let param = CreateFineTuneParamBuilder::new("training_file")
///         .validation_file("validation_file")
///         .model("curie")
///         .n_epochs(4)
///         .batch_size(1)
///         .learning_rate_multiplier(1.0)
///         .prompt_loss_weight(0.01)
///         .compute_classification_metrics(false)
///         .classification_n_classes(2)
///         .classification_positive_class("positive")
///         .classification_betas(vec![0.5, 0.5])
///         .suffix("suffix")
///         .build()?;
///
///     let resp = create(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client, param: &CreateFineTuneParam) -> Result<FineTune> {
    client.create_fine_tune(param).await
}

/// List your organization's fine-tuning jobs.
///
/// Related OpenAI docs: [List Fine-tune](https://beta.openai.com/docs/api-reference/fine-tunes/list)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::list};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let client = Client::new();
///
///     let resp = list(&client).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn list(client: &Client) -> Result<ListFineTune> {
    client.list_fine_tune().await
}

/// Gets info about the fine-tune job.
///
/// Related OpenAI docs: [Retrieve Fine-tune](https://beta.openai.com/docs/api-reference/fine-tunes/retrieve)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::retrieve};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let client = Client::new();
///
///     let resp = retrieve(&client, "ft-123").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
pub async fn retrieve(client: &Client, fine_tune_id: impl Into<String>) -> Result<FineTune> {
    client.retrieve_fine_tune(fine_tune_id.into()).await
}

/// Immediately cancel a fine-tune job.
///
/// Related OpenAI docs: [Cancel Fine-tune](https://beta.openai.com/docs/api-reference/fine-tunes/cancel)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::cancel};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let resp = cancel(&client, "ft-123").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn cancel(client: &Client, fine_tune_id: impl Into<String>) -> Result<FineTune> {
    client.cancel_fine_tune(fine_tune_id.into()).await
}

/// Get fine-grained status updates for a fine-tune job.
///
/// Related OpenAI docs: [List Fine-tune Events](https://beta.openai.com/docs/api-reference/fine-tunes/events)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::list_events};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let resp = list_events(&client, "ft-123").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn list_events(client: &Client, fine_tune_id: impl Into<String>) -> Result<ListEvents> {
    client.list_fine_tune_events(fine_tune_id.into()).await
}

/// Get a stream of fine-grained status updates for a fine-tune job.
///
/// Related OpenAI docs: [List Fine-tune Events](https://beta.openai.com/docs/api-reference/fine-tunes/events#fine-tunes/events-stream)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::list_events_with_stream};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let mut resp = list_events_with_stream(&client, "ft-123").await?;
///
///     while let Some(chunk) = resp.chunk().await? {
///         println!("{:#?}", chunk);
///     }
///
///     Ok(())
/// }
pub async fn list_events_with_stream(
    client: &Client,
    fine_tune_id: impl Into<String>,
) -> Result<reqwest::Response> {
    client
        .list_fine_tune_events_with_stream(fine_tune_id.into())
        .await
}

/// Delete a fine-tuned model. You must have the Owner role in your organization.
///
/// Related OpenAI docs: [Delete Fine-tuned model](https://beta.openai.com/docs/api-reference/fine-tunes/delete-model)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, fine_tune::delete};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let resp = delete(&client, "model-to-delete").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn delete<T: Into<String>>(client: &Client, model: T) -> Result<Delete> {
    client.delete_fine_tune(model.into()).await
}

impl Client {
    async fn create_fine_tune(&self, param: &CreateFineTuneParam) -> Result<FineTune> {
        self.post::<CreateFineTuneParam, FineTune>("fine-tunes", Some(param))
            .await
    }

    async fn list_fine_tune(&self) -> Result<ListFineTune> {
        self.get::<(), ListFineTune>("fine-tunes", None).await
    }

    async fn retrieve_fine_tune(&self, fine_tune_id: String) -> Result<FineTune> {
        self.get::<(), FineTune>(&format!("fine-tunes/{fine_tune_id}"), None)
            .await
    }

    async fn cancel_fine_tune(&self, fine_tune_id: String) -> Result<FineTune> {
        self.post::<(), FineTune>(&format!("fine-tunes/{fine_tune_id}/cancel"), None)
            .await
    }

    async fn list_fine_tune_events(&self, fine_tune_id: String) -> Result<ListEvents> {
        self.get::<(), ListEvents>(&format!("fine-tunes/{fine_tune_id}/events"), None)
            .await
    }

    async fn list_fine_tune_events_with_stream(
        &self,
        fine_tune_id: String,
    ) -> Result<reqwest::Response> {
        self.get_stream::<serde_json::Value>(
            &format!("fine-tunes/{fine_tune_id}/events"),
            Some(&json!({"stream": true})),
        )
        .await
    }

    async fn delete_fine_tune(&self, model: String) -> Result<Delete> {
        self.delete::<(), Delete>(&format!("models/{model}"), None)
            .await
    }
}

#[cfg(test)]
mod tests {}
