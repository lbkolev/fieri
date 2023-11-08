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
use std::cell::Cell;

use crate::{
    api_resources::{Choices, TokenUsage},
    Client, Result,
};

/// Parameters for [`Create Completion`](create) request.
#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct CompletionParam {
    /// The model to use for the completion request.
    model: String,

    /// The prompt(s) to generate completions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,

    /// The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,

    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,

    /// Higher values means the model will take more risks.
    ///
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It's generally recommended to alter this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    /// How many completions to generate for each prompt.
    ///
    /// Note: Because this parameter generates many completions, it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,

    // Whether to stream back partial progress.
    //
    // For streamed progress, use [`create_with_stream`](create_with_stream).
    #[builder(setter(skip))]
    stream: Cell<bool>,

    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<f32>,

    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    echo: Option<bool>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    ///
    /// The returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,

    /// Number between -2.0 and 2.0.
    ///
    /// Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0.
    ///
    /// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    /// Generates best_of completions server-side and returns the "best" (the one with the highest log probability per token).
    ///
    /// Results cannot be streamed.
    #[serde(skip_serializing_if = "Option::is_none")]
    best_of: Option<u16>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl CompletionParamBuilder {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: Some(model.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create completion`](create) request.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Completion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choices>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

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
pub async fn create_with_stream(
    client: &Client,
    param: &CompletionParam,
) -> Result<reqwest::Response> {
    param.stream.set(true);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_completion() {
        let param: CompletionParam = serde_json::from_str(
            r#"
            {
                "model": "text-davinci-003",
                "prompt": "Say this is a test",
                "max_tokens": 7,
                "temperature": 0,
                "top_p": 1,
                "n": 1,
                "stream": false,
                "logprobs": null,
                "stop": "\n"
            }
            "#,
        )
        .unwrap();

        let resp: Completion = serde_json::from_str(
            r#"
            {
                "id": "cmpl-uqkvlQyYK7bGYrRHQ0eXlWi7",
                "object": "text_completion",
                "created": 1589478378,
                "model": "text-davinci-003",
                "choices": [
                {
                    "text": "\n\nThis is indeed a test",
                    "index": 0,
                    "logprobs": null,
                    "finish_reason": "length"
                }
                ],
                "usage": {
                    "prompt_tokens": 5,
                    "completion_tokens": 7,
                    "total_tokens": 12
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(param.model, "text-davinci-003");
        assert_eq!(param.prompt.unwrap(), "Say this is a test");
        assert_eq!(param.suffix, None);
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(
            resp.choices[0].text,
            Some("\n\nThis is indeed a test".to_string())
        );
        assert_eq!(resp.choices[0].logprobs, None);
        assert_eq!(resp.usage.unwrap().prompt_tokens, 5);
    }
}
