//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.
//!
//! The completions endpoint can be used for a wide variety of tasks. It provides a simple but powerful interface to any of the available models.
//! You input some text as a prompt, and the model will generate a text completion that attempts to match whatever context or pattern you gave it.
//!
//! For example, if you give the API the prompt, "As Descartes said, I think, therefore", it will return the completion " I am" with high probability.
//!
//! models can do everything from generating original stories to performing complex text analysis.
//! Because they can do so many things, you have to be explicit in describing what you want.
//!
//! Showing, not just telling, is often the secret to a good prompt.

use derive_builder::Builder;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    types::{Completion, CompletionParam},
    Client, Result,
};

/// Creates a completion for the provided prompt and parameters.
///
/// The completion API is the most powerful Endpoint in the OpenAI API.
///
/// It can be used to generate structured data like `JSON`, `HTML`, `LaTeX`, code in any programming language and more.
///
/// Related OpenAI docs: [Create Completions](https://beta.openai.com/docs/api-reference/completions/create)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, completion::{create, CompletionParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let param = CompletionParamBuilder::new("ada")
///         .prompt("Haskell is a programming language. Generate a poem about Messi and World Cup 2022.")
///         .temperature(0.5)
///         .build()?;
///
///     let resp = create(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
#[deprecated(
    since = "0.7.0",
    note = "Please use chat endpoint. More at https://platform.openai.com/docs/guides/text-generation/completions-api"
)]
pub async fn create(client: &Client, param: &CompletionParam) -> Result<Completion> {
    client.create_completion(param).await
}

/// Creates a completion stream for the provided prompt and parameters.
///
/// Related OpenAI docs: [Create Completions](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stream)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, completion::{create_with_stream, CompletionParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
///
///     let param = CompletionParamBuilder::new("ada")
///         .prompt("Haskell is a programming language. Generate a poem about Messi and World Cup 2022.")
///         .temperature(0.5)
///         .build()?;
///
///     let mut resp = create_with_stream(&client, &param).await?;
///
///     while let Some(chunk) = resp.chunk().await? {
///         let val = String::from_utf8(chunk.to_vec())?;
///         println!("{}", val);
///     }
///
///     Ok(())
/// }
#[deprecated(
    since = "0.7.0",
    note = "Please use chat endpoint. More at https://platform.openai.com/docs/guides/text-generation/completions-api"
)]
pub async fn create_with_stream(
    client: &Client,
    param: &CompletionParam,
) -> Result<reqwest::Response> {
    client.create_completion_with_stream(param).await
}

impl Client {
    async fn create_completion(&self, param: &CompletionParam) -> Result<Completion> {
        self.post::<CompletionParam, Completion>("completions", Some(param))
            .await
    }

    async fn create_completion_with_stream(
        &self,
        param: &CompletionParam,
    ) -> Result<reqwest::Response> {
        self.post_stream("completions", Some(param)).await
    }

    /*
    fn create_completion_with_stream(
        &self,
        param: &CompletionParam,
    ) -> Pin<
        Box<
            dyn Stream<
                    Item = std::result::Result<Completion, Box<dyn std::error::Error + Send + '_>>,
                > + Send,
        >,
    > {
        Box::pin(stream! {
            let mut resp = match self.post_stream("completions", Some(&param)).await {
                Ok(r) => r,
                Err(e) => {
                    yield Err(Box::new(e) as Box<dyn std::error::Error + Send + '_>);
                    return;
                }
            };

            let mut cv = String::new();

            while let Ok(Some(chunk)) = resp.chunk().await {
                let a = match String::from_utf8(chunk.to_vec()) {
                    Ok(s) => s,
                    Err(e) => {
                        yield Err(Box::new(e) as Box<dyn std::error::Error + Send + '_>);
                        continue;
                    }
                };
                let whole_val = a.split("data: ").collect::<Vec<_>>();

                for part in whole_val {
                    match serde_json::from_str::<Completion>(part) {
                        Ok(v) => yield Ok(v),
                        Err(_) => {
                            cv.push_str(part);
                            if let Ok(v) = serde_json::from_str::<Completion>(&cv) {
                                cv.clear();
                                yield Ok(v);
                            }
                        }
                    }
                }
            }
        })
    }
    */
}

#[cfg(test)]
mod tests {}
