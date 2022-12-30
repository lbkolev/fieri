//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Embeddings are most commonly used for:
//! - Search (where results are ranked by relevance to a query string)
//! - Clustering (where text strings are grouped by similarity)
//! - Recommendations (where items with related text strings are recommended)
//! - Anomaly detection (where outliers with little relatedness are identified)
//! - Diversity measurement (where similarity distributions are analyzed)
//! - Classification (where text strings are classified by their most similar label)

use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    api_resources::{RequestError, TokenUsage},
    Client, Result,
};

/// Parameters for [`Create Embedding`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct EmbeddingParam {
    /// The model to use for the embedding request.
    model: String,

    /// Input text to get embeddings for, encoded as a string.
    ///
    /// Each input must not exceed 8192 tokens in length.
    input: String,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    user: Option<String>,
}

impl EmbeddingParamBuilder {
    pub fn new(model: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            model: Some(model.into()),
            input: Some(input.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create Embedding`](create) request.
#[derive(Clone, Debug, Default, Deserialize, Getters)]
#[serde(default)]
pub struct Embedding {
    object: String,
    data: Vec<EmbeddingData>,
    mode: String,

    usage: Option<TokenUsage>,
    error: Option<RequestError>,
}

/// The distance between two vectors measures their relatedness. Small distances suggest high relatedness and large distances suggest low relatedness.
#[derive(Clone, Debug, Deserialize, Getters)]
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
/// use fieri::{Client, embedding::{create, EmbeddingParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = EmbeddingParamBuilder::new("text-embedding-ada-002", "Hello world!")
///         .user("/user/")
///         .build()?;
///
///     let resp = create(&client, &param).await?;
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

        let param = EmbeddingParamBuilder::default()
            .model("text-embedding-ada-002")
            .input("Hello world!")
            .user("rand-user")
            .build()?;

        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.error().is_none());
        assert!(resp.usage().is_some());
        assert!(resp.mode().is_empty());
        Ok(())
    }
}
