//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{api_resources::model::Model, client::Client, Result};

///
#[derive(Debug, Getters, Serialize)]
pub struct CompletionParam {
    model: String,

    prompt: String,

    suffix: Option<String>,

    max_tokens: u16,

    temperature: f32,

    top_p: f32,

    n: u32,

    // TODO add stream
    // Whether to stream back partial progress.
    // stream: Option<bool>,
    logprobs: Option<f32>,

    echo: bool,

    stop: Option<String>,

    presence_penalty: f32,

    frequency_penalty: f32,

    best_of: u16,

    logit_bias: HashMap<String, i8>,

    user: String,
}

impl Default for CompletionParam {
    fn default() -> Self {
        Self {
            model: Model::None.to_string(),
            prompt: "<|endoftext|>".to_string(),
            suffix: None,
            max_tokens: 16,
            temperature: 1.0,
            top_p: 1.0,
            n: 1,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// ID of the model to use. You can use the List models API to see all of your available models.
    pub fn add_model(mut self, model: Model) -> Self {
        self.model = model.to_string();

        self
    }

    /// The prompt(s) to generate completions for, encoded as a string.
    pub fn add_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;

        self
    }

    /// The suffix that comes after a completion of inserted text.
    pub fn add_suffix(mut self, suffix: Option<String>) -> Self {
        self.suffix = suffix;

        self
    }

    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    pub fn add_maxtokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = max_tokens;

        self
    }

    /// Higher values means the model will take more risks.
    ///
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    pub fn add_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;

        self
    }

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It's generally recommended to alter this or `temperature` but not both.
    pub fn add_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;

        self
    }

    /// How many completions to generate for each prompt.
    ///
    /// Note: Because this parameter generates many completions, it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    pub fn add_n(mut self, n: u32) -> Self {
        self.n = n;

        self
    }

    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens.
    pub fn add_logprobs(mut self, logprobs: Option<f32>) -> Self {
        self.logprobs = logprobs;

        self
    }

    /// Echo back the prompt in addition to the completion
    pub fn add_echo(mut self, echo: bool) -> Self {
        self.echo = echo;

        self
    }

    /// Up to 4 sequences where the API will stop generating further tokens.
    ///
    /// The returned text will not contain the stop sequence.
    pub fn add_stop(mut self, stop: Option<String>) -> Self {
        self.stop = stop;

        self
    }

    /// Number between -2.0 and 2.0.
    ///
    /// Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    pub fn add_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;

        self
    }

    /// Number between -2.0 and 2.0.
    ///
    /// Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    pub fn add_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = frequency_penalty;

        self
    }

    /// Generates best_of completions server-side and returns the "best" (the one with the highest log probability per token).
    ///
    /// Results cannot be streamed.
    pub fn add_best_of(mut self, best_of: u16) -> Self {
        self.best_of = best_of;

        self
    }

    /// Modify the likelihood of specified tokens appearing in the completion.
    pub fn add_logit_bias(mut self, logit_bias: HashMap<String, i8>) -> Self {
        self.logit_bias = logit_bias;

        self
    }

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub fn add_user(mut self, user: String) -> Self {
        self.user = user;

        self
    }
}

#[derive(Debug, Getters, Deserialize)]
pub struct CompletionResp {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Option<Vec<Choices>>,
    usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

#[derive(Debug, Getters, Deserialize)]
pub struct ErrorResp {
    message: Option<String>,
    r#type: Option<String>,
    param: Option<String>,
    code: Option<i32>,
}

#[derive(Debug, Getters, Deserialize)]
pub struct Choices {
    text: String,
    index: u32,
    logprobs: Option<bool>,
    finish_reason: String,
}

#[derive(Debug, Getters, Deserialize)]
pub struct TokenUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
}

/// Creates a completion for the provided prompt and parameters
///
/// Example:
/// ```rust
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::{
///         model::Model,
///         completion::{
///             create_completion,
///             CompletionParam,
///             CompletionResp,
///         }
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let param = CompletionParam::new()
///         .add_model(Model::Ada)
///         .add_prompt("sup?".to_string());
///     let resp: CompletionResp = create_completion(&client, param).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn create_completion(
    client: &Client<'_>,
    param: CompletionParam,
) -> Result<CompletionResp> {
    client.create_completion(param).await
}

impl<'a> Client<'a> {
    async fn create_completion(&self, param: CompletionParam) -> Result<CompletionResp> {
        let resp = self
            .post::<CompletionParam, CompletionResp>("/completions".to_string(), Some(param))
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{api_resources::completion::CompletionParam, client::Client, config::Config};
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create_completion() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let param = CompletionParam::new()
            .add_model(Model::CurieInstructBeta)
            .add_prompt("Generate a complex and elaborate 'Hello, World' in R.".to_string());

        println!("{:#?}", param);
        let resp = create_completion(&client, param).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
        Ok(())
    }
}
