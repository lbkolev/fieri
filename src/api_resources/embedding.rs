//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Embeddings are most commonly used for:
//! - Search (where results are ranked by relevance to a query string)
//! - Clustering (where text strings are grouped by similarity)
//! - Recommendations (where items with related text strings are recommended)
//! - Anomaly detection (where outliers with little relatedness are identified)
//! - Diversity measurement (where similarity distributions are analyzed)
//! - Classification (where text strings are classified by their most similar label)

use crate::{
    types::{Embedding, EmbeddingParam},
    Client, Result,
};

/// Creates an embedding vector representing the input text.
///
/// Related OpenAI docs: [Create Embeddings](https://beta.openai.com/docs/api-reference/embeddings/create).
///
/// ## Example
/// ```no_run
/// use fieri::{Client, embedding::{create, EmbeddingParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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
mod tests {}
