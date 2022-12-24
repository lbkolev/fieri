//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    api_resources::{Choices, ErrorResp, TokenUsage},
    Client, Models, Result,
};

/// Parameters for [`create`](crate::api_resources::completion::create) completion request.
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
    pub stream: bool,

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: Option<Models>) -> Self {
        self.model = model;

        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;

        self
    }

    pub fn suffix(mut self, suffix: Option<String>) -> Self {
        self.suffix = suffix;

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

    pub fn stream(mut self, stream: bool) -> Self {
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

    pub fn stop(mut self, stop: Option<String>) -> Self {
        self.stop = stop;

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

    pub fn user(mut self, user: String) -> Self {
        self.user = user;

        self
    }
}

/// Response from [`create`](crate::api_resources::completion::create) completion reqest.
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
/// Related OpenAI docs: [Create Completions](https://beta.openai.com/docs/api-reference/completions/create)
///
/// The completion API is the most powerful Endpoint in the OpenAI API.
///
/// It can be used to generate text, images, audio, and video.
///
/// It can also be used to generate structured data like JSON, HTML, or LaTeX.
///
/// It can be used to generate code in any programming language.
///
/// It can be used to generate log messages, email replies, tweets, and more.
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     Models,
///     client::Client,
///     config::Config,
///     api_resources::completion::{
///         create,
///         CompletionParam,
///         CompletionResp,
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let param = CompletionParam::new()
///         .model(Some(Models::Ada))
///         .prompt("Generate a complex and unintuitive 'Hello, World' example in Haskell.".to_string())
///         .temperature(0.5);
///     let resp: CompletionResp = create(&client, &param).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client<'_>, param: &CompletionParam) -> Result<CompletionResp> {
    client.create_completion(param).await
}

/*pub async fn create_with_stream(client: &Client<'_>, param: &CompletionParam) -> Result<CompletionResp> {
    client.create_completion_with_stream(param).await
}*/

impl<'a> Client<'a> {
    async fn create_completion(&self, param: &CompletionParam) -> Result<CompletionResp> {
        let resp = self
            .post::<CompletionParam, CompletionResp>("/completions".to_string(), Some(param))
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
    use crate::Config;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create() -> Result<()> {
        let config = Config::new(env::var("OPENAI_API_KEY")?);
        let client = Client::new(&config);

        let param = CompletionParam::new()
            .model(Some(Models::CurieInstructBeta))
            .prompt("Generate a complex and elaborate 'Hello, World' in R.".to_string());

        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.error().is_none(), true);
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
