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
use std::{borrow::Cow, path::Path};

use crate::{
    api_resources::{Delete, ErrorResp, File, Files, TokenUsage},
    Client, Result,
};

/// Parameters for [`Create Fine-tune`](create) request.
#[derive(Debug, Serialize)]
pub struct CreateFineTuneParam {
    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload](fieri::file::upload) file for how to upload a file.
    pub training_file: String,

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in the fine-tuning results file.
    /// Your train and validation data should be mutually exclusive.
    // Note: Even though it's given as "optional" in the docs, it's required in the API.
    pub validation_file: String,

    /// The name of the base model to fine-tune. You can select one of "ada", "babbage", "curie", "davinci", or a fine-tuned model created after 2022-04-21.
    pub model: crate::Models,

    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    pub n_epochs: u32,

    /// The batch size to use for training. The batch size is the number of training examples used to train a single forward and backward pass.
    pub batch_size: Option<u32>,

    /// The learning rate multiplier to use for training. The fine-tuning learning rate is the original learning rate used for pretraining multiplied by this value.
    pub learning_rate_multiplier: Option<f32>,

    /// The weight to use for loss on the prompt tokens.
    ///
    /// This controls how much the model tries to learn to generate the prompt (as compared to the completion which always has a weight of 1.0), and can add a stabilizing effect to training when completions are short.
    pub prompt_loss_weight: f32,

    /// If set, we calculate classification-specific metrics such as accuracy and F-1 score using the validation set at the end of every epoch.
    pub compute_classification_metrics: bool,

    /// The number of classes in a classification task.
    ///
    /// This parameter is required for multiclass classification.
    pub classification_n_classes: Option<u32>,

    /// The positive class in binary classification.
    ///
    /// This parameter is needed to generate precision, recall, and F1 metrics when doing binary classification.
    pub classification_positive_class: Option<String>,

    /// If this is provided, we calculate F-beta scores at the specified beta values. The F-beta score is a generalization of F-1 score. This is only used for binary classification.
    ///
    /// With a beta of 1 (i.e. the F-1 score), precision and recall are given the same weight. A larger beta score puts more weight on recall and less on precision. A smaller beta score puts more weight on precision and less on recall.
    pub classification_betas: Option<Vec<f32>>,

    /// Suffix to be added to the model's name.
    pub suffix: String,
}

impl CreateFineTuneParam {
    pub fn new<X, Y>(training_file: X, validation_file: Y) -> Self
    where
        X: Into<String>,
        Y: Into<String>,
    {
        Self {
            training_file: training_file.into(),
            validation_file: validation_file.into(),
            model: crate::Models::Curie,
            n_epochs: 4,
            batch_size: Some(1),
            learning_rate_multiplier: None,
            prompt_loss_weight: 0.01,
            compute_classification_metrics: false,
            classification_n_classes: None,
            classification_positive_class: None,
            classification_betas: None,
            suffix: "".to_string(),
        }
    }

    pub fn model(mut self, model: crate::Models) -> Self {
        self.model = model;

        self
    }

    pub fn n_epochs(mut self, n_epochs: u32) -> Self {
        self.n_epochs = n_epochs;

        self
    }

    pub fn batch_size(mut self, batch_size: u32) -> Self {
        self.batch_size = Some(batch_size);

        self
    }

    pub fn learning_rate_multiplier(mut self, learning_rate_multiplier: f32) -> Self {
        self.learning_rate_multiplier = Some(learning_rate_multiplier);

        self
    }

    pub fn prompt_loss_weight(mut self, prompt_loss_weight: f32) -> Self {
        self.prompt_loss_weight = prompt_loss_weight;

        self
    }

    pub fn compute_classification_metrics(mut self, compute_classification_metrics: bool) -> Self {
        self.compute_classification_metrics = compute_classification_metrics;

        self
    }

    pub fn classification_n_classes(mut self, classification_n_classes: u32) -> Self {
        self.classification_n_classes = Some(classification_n_classes);

        self
    }

    pub fn classification_positive_class<T: Into<String>>(
        mut self,
        classification_positive_class: T,
    ) -> Self {
        self.classification_positive_class = Some(classification_positive_class.into());

        self
    }

    pub fn classification_betas(mut self, classification_betas: Vec<f32>) -> Self {
        self.classification_betas = Some(classification_betas);

        self
    }

    pub fn suffix<T: Into<String>>(mut self, suffix: T) -> Self {
        self.suffix = suffix.into();

        self
    }
}

/// Response from [`Create Fine-Tune`][create] request.
#[derive(Debug, Deserialize, Getters)]
pub struct FineTune {
    id: Option<String>,
    object: Option<String>,
    model: Option<String>,
    created_at: Option<u64>,
    events: Option<Events>,
    fine_tuned_model: Option<String>,
    hyperparams: Option<HyperParams>,
    organization_id: Option<String>,
    result_files: Option<Files>,
    validation_files: Option<Files>,
    training_files: Option<Files>,
    status: Option<String>,
    updated_at: Option<u64>,

    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Hyper parameters for fine-tuning a model.
#[derive(Debug, Deserialize, Getters)]
pub struct HyperParams {
    n_epochs: u32,
    batch_size: u32,
    learning_rate_multiplier: f32,
    prompt_loss_weight: f32,
    compute_classification_metrics: Option<bool>,
    classification_n_classes: Option<u32>,
    classification_positive_class: Option<String>,
    classification_betas: Option<Vec<f32>>,
}

/// Events occuring on Fine-tunes
#[derive(Debug, Deserialize, Getters)]
pub struct Event {
    object: String,
    created_at: u64,
    level: String,
    message: String,
}

type Events = Vec<Event>;

#[derive(Debug, Deserialize, Getters)]
pub struct ListEvents {
    object: Option<String>,
    data: Option<Vec<Event>>,

    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct ListFineTune {
    object: Option<String>,
    data: Option<Vec<FineTune>>,

    token_usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Creates a job that fine-tunes a specified model from a given dataset.
///
/// Related OpenAI docs: [Create Fine-tune](https://beta.openai.com/docs/api-reference/fine-tunes/create)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, fine_tune::{create, CreateFineTuneParam}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///     let param = CreateFineTuneParam::new("training_file", "validation_file")
///     .model(fieri::Models::Curie)
///     .n_epochs(4)
///     .batch_size(1)
///     .learning_rate_multiplier(1.0)
///     .prompt_loss_weight(0.01)
///     .compute_classification_metrics(false)
///     .classification_n_classes(2)
///     .classification_positive_class("positive")
///     .classification_betas(vec![0.5, 0.5])
///     .suffix("suffix");
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
/// ```rust
/// use std::env;
/// use fieri::{Client, fine_tune::list};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let client = Client::new(env::var("OPENAI_API_KEY")?);
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
/// use std::env;
/// use fieri::{Client, fine_tune::retrieve};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let resp = retrieve(&client, "ft-123").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
pub async fn retrieve<T: Into<String>>(client: &Client, fine_tune_id: T) -> Result<FineTune> {
    client.retrieve_fine_tune(fine_tune_id).await
}

/// Immediately cancel a fine-tune job.
///
/// Related OpenAI docs: [Cancel Fine-tune](https://beta.openai.com/docs/api-reference/fine-tunes/cancel)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, fine_tune::cancel};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let resp = cancel(&client, "ft-123").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn cancel<T: Into<String>>(client: &Client, fine_tune_id: T) -> Result<FineTune> {
    client.cancel_fine_tune(fine_tune_id).await
}

/// Get fine-grained status updates for a fine-tune job.
///
/// Related OpenAI docs: [List Fine-tune Events](https://beta.openai.com/docs/api-reference/fine-tunes/events)
///
/// ## Example
/// ```no_run
/// use std::env;
/// use fieri::{Client, fine_tune::list_events};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let resp = list_events(&client, "ft-123").await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn list_events<T: Into<String>>(client: &Client, fine_tune_id: T) -> Result<ListEvents> {
    client.list_fine_tune_events(fine_tune_id).await
}

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
    async fn create_fine_tune(&self, param: &CreateFineTuneParam) -> Result<FineTune> {
        self.post::<CreateFineTuneParam, FineTune>("fine-tunes", Some(param))
            .await
    }

    async fn list_fine_tune(&self) -> Result<ListFineTune> {
        self.get::<(), ListFineTune>("fine-tunes", None).await
    }

    async fn retrieve_fine_tune<T: Into<String>>(&self, fine_tune_id: T) -> Result<FineTune> {
        self.get::<(), FineTune>(format!("fine-tunes/{}", fine_tune_id.into()).as_str(), None)
            .await
    }

    async fn cancel_fine_tune<T: Into<String>>(&self, fine_tune_id: T) -> Result<FineTune> {
        self.post::<(), FineTune>(
            format!("fine-tunes/{}/cancel", fine_tune_id.into()).as_str(),
            None,
        )
        .await
    }

    async fn list_fine_tune_events<T: Into<String>>(&self, fine_tune_id: T) -> Result<ListEvents> {
        self.get::<(), ListEvents>(
            format!("fine-tunes/{}/events", fine_tune_id.into()).as_str(),
            None,
        )
        .await
    }

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

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_fine_tune() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = CreateFineTuneParam::new(
            "file-mN8td2DLg8bHQh0K7Bla7x7Z",
            "file-1FZQ73L5AK8UknTTT0PxWMBE",
        )
        .n_epochs(1)
        .batch_size(1)
        .learning_rate_multiplier(1.0)
        .prompt_loss_weight(1.0)
        .compute_classification_metrics(true)
        .classification_n_classes(1)
        .classification_positive_class("positive")
        .classification_betas(vec![1.0, 1.0])
        .suffix(" ");

        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list_fine_tune() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = list(&client).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_retrieve_fine_tune() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = retrieve(&client, "ft-pxhz75Q1U9cAHOyCRzaoClNL").await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_list_fine_tune_events() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = list_events(&client, "ft-pxhz75Q1U9cAHOyCRzaoClNL").await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }

    #[ignore]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_cancel_fine_tune() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = cancel(&client, "ft-pxhz75Q1U9cAHOyCRzaoClNL").await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }

    #[ignore = "requires file deletion"]
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_delete_fine_tune() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let resp = delete(&client, "model-to-delete").await?;
        println!("{:#?}", resp);

        assert!(resp.deleted().is_some());
        assert!(resp.error().is_some());
        Ok(())
    }
}
