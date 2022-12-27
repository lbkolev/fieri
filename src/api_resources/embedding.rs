//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Embeddings are most commonly used for:
//! - Search (where results are ranked by relevance to a query string)
//! - Clustering (where text strings are grouped by similarity)
//! - Recommendations (where items with related text strings are recommended)
//! - Anomaly detection (where outliers with little relatedness are identified)
//! - Diversity measurement (where similarity distributions are analyzed)
//! - Classification (where text strings are classified by their most similar label)

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{ErrorResp, TokenUsage},
    Client, Models, Result,
};

/// Parameters for [`Create Embedding`](create) request.
#[derive(Debug, Clone, Serialize)]
pub struct EmbeddingParam {
    /// The model to use for the embedding request.
    ///
    /// The available models can be found [`here`](crate::Models).
    pub model: Option<Models>,

    /// Input text to get embeddings for, encoded as a string.
    ///
    /// Each input must not exceed 8192 tokens in length.
    // TODO: To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays.
    pub input: String,

    pub user: String,
}

impl EmbeddingParam {
    pub fn new<T: Into<String>>(model: Models, input: T) -> Self {
        Self {
            model: Some(model),
            input: input.into(),
            user: String::new(),
        }
    }

    pub fn user<T: Into<String>>(mut self, user: T) -> Self {
        self.user = user.into();

        self
    }
}

/// Response from [`Create Embedding`](create) request.
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct Embedding {
    object: Option<String>,
    data: Option<Vec<EmbeddingData>>,
    mode: Option<String>,
    usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// The distance between two vectors measures their relatedness. Small distances suggest high relatedness and large distances suggest low relatedness.
#[derive(Debug, Clone, Deserialize, Getters)]
pub struct EmbeddingData {
    object: String,
    embedding: Embeddings,
    index: u64,
}

type Embeddings = Vec<f32>;

/// Creates an embedding vector representing the input text.
///
/// Related OpenAI docs: [Create Embeddings](https://beta.openai.com/docs/api-reference/embeddings/create).
///
/// ## Example
/// ```rust
/// use std::env;
/// use fieri::{
///     Client, Models,
///     embedding::{create, EmbeddingParam, Embedding},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = EmbeddingParam::new(Models::TextEmbeddingAda002, "Hello world!");
///     let resp: Embedding = create(&client, &param).await?;
///     println!("{:?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client, param: &EmbeddingParam) -> Result<Embedding> {
    client.create_embeddings(param).await
}

impl Client {
    async fn create_embeddings(&self, param: &EmbeddingParam) -> Result<Embedding> {
        self.post::<EmbeddingParam, Embedding>("embeddings", Some(param))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = EmbeddingParam::new(Models::TextEmbeddingAda002, "Hello world!");
        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        Ok(())
    }
}
