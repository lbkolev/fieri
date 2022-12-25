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

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    api_resources::{Choices, ErrorResp, TokenUsage},
    Client, Models, Result,
};

/// Parameters for [`Create Completion`](create) request.
#[derive(Debug, Serialize)]
pub struct CompletionParam {
    /// The model to use for the completion request.
    ///
    /// The available models can be found [`here`](crate::Models).
    pub model: Option<Models>,

    /// The prompt(s) to generate completions for.
    pub prompt: String,

    /// The suffix that comes after a completion of inserted text.
    pub suffix: Option<String>,

    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    pub max_tokens: u16,

    /// Higher values means the model will take more risks.
    ///
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    pub temperature: f32,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It's generally recommended to alter this or `temperature` but not both.
    pub top_p: f32,

    /// How many completions to generate for each prompt.
    ///
    /// Note: Because this parameter generates many completions, it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    pub n: u32,

    // Whether to stream back partial progress.
    // defaults to false.
    // For streamed progress, use [`create_with_stream`](create_with_stream).
    stream: bool,

    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens.
    pub logprobs: Option<f32>,

    /// Echo back the prompt in addition to the completion
    pub echo: bool,

    /// Up to 4 sequences where the API will stop generating further tokens.
    ///
    /// The returned text will not contain the stop sequence.
    pub stop: Option<String>,

    /// Number between -2.0 and 2.0.
    ///
    /// Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    pub presence_penalty: f32,

    /// Number between -2.0 and 2.0.
    ///
    /// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    pub frequency_penalty: f32,

    /// Generates best_of completions server-side and returns the "best" (the one with the highest log probability per token).
    ///
    /// Results cannot be streamed.
    pub best_of: u16,

    /// Modify the likelihood of specified tokens appearing in the completion.
    pub logit_bias: HashMap<String, i8>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub user: String,
}

impl Default for CompletionParam {
    fn default() -> Self {
        Self {
            model: None,
            prompt: "<|endoftext|>".to_string(),
            suffix: None,
            max_tokens: 16,
            temperature: 1.0,
            top_p: 1.0,
            n: 1,
            stream: false,
            logprobs: None,
            echo: false,
            stop: None,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            best_of: 1,
            logit_bias: HashMap::<String, i8>::new(),
            user: String::new(),
        }
    }
}

impl CompletionParam {
    pub fn new(model: Models) -> Self {
        Self {
            model: Some(model),
            ..Self::default()
        }
    }

    pub fn prompt<T: Into<String>>(mut self, prompt: T) -> Self {
        self.prompt = prompt.into();

        self
    }

    pub fn suffix<T: Into<String>>(mut self, suffix: Option<T>) -> Self {
        self.suffix = suffix.map(|t| t.into());

        self
    }

    pub fn max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = max_tokens;

        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;

        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;

        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = n;

        self
    }

    // TODO:
    #[allow(dead_code)]
    fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;

        self
    }

    pub fn logprobs(mut self, logprobs: Option<f32>) -> Self {
        self.logprobs = logprobs;

        self
    }

    pub fn echo(mut self, echo: bool) -> Self {
        self.echo = echo;

        self
    }

    pub fn stop<T: Into<String>>(mut self, stop: Option<T>) -> Self {
        self.stop = stop.map(|t| t.into());

        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;

        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = frequency_penalty;

        self
    }

    pub fn best_of(mut self, best_of: u16) -> Self {
        self.best_of = best_of;

        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<String, i8>) -> Self {
        self.logit_bias = logit_bias;

        self
    }

    pub fn user<T: Into<String>>(mut self, user: T) -> Self {
        self.user = user.into();

        self
    }
}

/// Response from [`Create completion`](create) request.
#[derive(Debug, Getters, Deserialize)]
pub struct Completion {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<Choices>>,
    usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/*
impl Iterator for CompletionResp {
    type Item = Vec<Choices>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(stream) = self.choices.as_mut() {
            if let Some(choice) = stream.pop() {
                Some(choice)
            } else {
                None
            }
        }
    }
}
*/

/// Creates a completion for the provided prompt and parameters.
///
/// The completion API is the most powerful Endpoint in the OpenAI API.
///
/// It can be used to generate structured data like `JSON`, `HTML`, `LaTeX`, code in any programming language and more.
///
/// Related OpenAI docs: [Create Completions](https://beta.openai.com/docs/api-reference/completions/create)
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     Client, Models,
///     completion::{create, CompletionParam, Completion},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = CompletionParam::new(Models::Ada)
///         .prompt("Haskell is a programming language. Generate a complex and unintuitive 'Hello, World' example in Haskell.")
///         .temperature(0.5);
///     let resp: Completion = create(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client, param: &CompletionParam) -> Result<Completion> {
    client.create_completion(param).await
}

/*pub async fn create_with_stream(client: &Client<'_>, param: &CompletionParam) -> Result<CompletionResp> {
    client.create_completion_with_stream(param).await
}*/

impl Client {
    async fn create_completion(&self, param: &CompletionParam) -> Result<Completion> {
        let resp = self
            .post::<CompletionParam, Completion>("completions", Some(param))
            .await?;

        Ok(resp)
    }

    /*
    async fn create_completion_with_stream(&self, param: &CompletionParam) -> Result<CompletionResp> {
        let resp = self
            .handler()
            .post(format!("{}{}", self.config().url(), "/completions".to_string()))
            .json(&param)
            .send()
            .await?;


        while let Some(chunk) = resp.chunk() {
            println!("Chunk: {:?}, chunk");
        }

        Ok(())
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = CompletionParam::new(Models::CurieInstructBeta).prompt(
            "R is a programming language. Generate a complex and elaborate 'Hello, World' in R.",
        );
        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.model().is_some());
        assert!(resp.error().is_none());
        Ok(())
    }

    /*async fn test_create_with_stream() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let param = CompletionParam::new()
            .model(Some(Model::CurieInstructBeta))
            .prompt("Generate a complex and elaborate 'Hello, World' in R.".to_string())
            .stream(true);

        let resp = create_with_stream(&client, &param).await?;
        while let Some(chunk) = resp.chunk().await? {

        }

        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        Ok(())
    }*/
}
