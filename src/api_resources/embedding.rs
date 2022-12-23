//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Embeddings are most commonly used for:
//! - Search (where results are ranked by relevance to a query string)
//! - Clustering (where text strings are grouped by similarity)
//! - Recommendations (where items with related text strings are recommended)
//! - Anomaly detection (where outliers with little relatedness are identified)
//! - Diversity measurement (where similarity distributions are analyzed)
//! - Classification (where text strings are classified by their most similar label)
//!

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{ErrorResp, TokenUsage},
    Client, Models, Result,
};

/// Parameters for [`create`](crate::api_resources::embedding::create) embedding request.
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbeddingParam {
    /// The model to use for the embedding request.
    ///
    /// The available models can be found [`here`](crate::Models).
    pub model: Option<Models>,

    /// Input text to get embeddings for, encoded as a string or array of tokens. To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays.
    ///
    /// Each input must not exceed 8192 tokens in length.
    pub input: String,

    pub user: String,
}

impl EmbeddingParam {
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

    pub fn user(mut self, user: String) -> Self {
        self.user = user;

        self
    }
}

/// Response from [`create`](crate::api_resources::embedding::create) embedding request.
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct EmbeddingResp {
    object: Option<String>,
    data: Option<Vec<EmbeddingData>>,
    mode: Option<String>,
    usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

#[derive(Debug, Clone, Deserialize, Getters)]
pub struct EmbeddingData {
    object: String,
    embedding: Embeddings,
    index: u64,
}

type Embeddings = Vec<f32>;

/// Creates an embedding vector representing the input text.
///
/// Related OpenAI docs: [Create embeddings](https://beta.openai.com/docs/api-reference/embeddings/create)
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     Models,
///     client::Client,
///     config::Config,
///     api_resources::embedding::{
///         create,
///         EmbeddingParam,
///         EmbeddingResp,
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let param = EmbeddingParam::new()
///         .model(Some(Models::TextEmbeddingAda002))
///         .input("..".to_string());
///     let resp: EmbeddingResp = create(&client, &param).await?;
///     println!("{:?}", resp);
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client<'_>, param: &EmbeddingParam) -> Result<EmbeddingResp> {
    client.create_embeddings(param).await
}

impl<'a> Client<'a> {
    async fn create_embeddings(&self, param: &EmbeddingParam) -> Result<EmbeddingResp> {
        let resp = self
            .post::<EmbeddingParam, EmbeddingResp>("/embeddings".to_string(), Some(param))
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let param = EmbeddingParam::new()
            .model(Some(Models::TextEmbeddingAda002))
            .input("fakdls,asdasdzxczxqs?".to_string());

        let resp = create(&client, &param).await?;

        println!("{:?}", resp);
        assert!(resp.error().is_none());
        Ok(())
    }
}
