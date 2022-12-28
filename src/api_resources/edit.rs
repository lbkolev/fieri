//! Given a prompt and an instruction, the model will return an edited version of the prompt.
//!
//! The edits endpoint can be used to edit text, rather than just completing it. You provide some text and an instruction for how to modify it.
//!
//! This is a natural interface for translating, editing, and tweaking text. This is also useful for refactoring and working with code.

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{Choices, RequestError, TokenUsage},
    Client, Models, Result,
};

/// Parameters for [`Create Edit`](create) request.
#[derive(Debug, Clone, Serialize)]
pub struct EditParam {
    /// The model to use for the edit request.
    ///
    /// The available models can be found [`here`](crate::Models).
    pub model: Option<Models>,

    /// The input text to use as a starting point for the edit.
    pub input: String,

    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,

    /// How many edits to generate for the input and instruction.
    pub n: u32,

    /// What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// It's recommended to alter this or `top_p` but not both.
    pub temperature: f32,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It's recommended to alter this or `temperature` but not both.
    pub top_p: f32,
}

impl Default for EditParam {
    fn default() -> Self {
        Self {
            model: None,
            input: '"'.to_string(),
            instruction: String::new(),
            n: 1,
            temperature: 1.0,
            top_p: 1.0,
        }
    }
}

impl EditParam {
    pub fn new<T: Into<String>>(model: Models, instruction: T) -> Self {
        Self {
            model: Some(model),
            instruction: instruction.into(),
            ..Self::default()
        }
    }

    pub fn input<T: Into<String>>(mut self, input: T) -> Self {
        self.input = input.into();

        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = n;

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
}

/// Response from [`Create Edit`](create) request.
#[derive(Debug, Deserialize, Getters)]
pub struct Edit {
    object: Option<String>,
    created: Option<u64>,
    choices: Option<Vec<Choices>>,
    usage: Option<TokenUsage>,
    error: Option<RequestError>,
}

/// Creates a new edit for the provided input, instruction, and parameters.
///
/// Related OpenAI docs: [Create an Edit](https://beta.openai.com/docs/api-reference/edits/create)
///
/// ## Example
/// ```rust
/// use std::env;
/// use fieri::{
///     Client, Models,
///     edit::{create, EditParam, Edit},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = EditParam::new(Models::TextDavinciEdit001, "Fix the spelling mistakes")
///         .input("What dey of the wek is it?");
///
///     let resp: Edit = create(&client, &param).await?;
///     println!("{:#?}", resp);
///
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client, param: &EditParam) -> Result<Edit> {
    client.create_edit(param).await
}

impl Client {
    async fn create_edit(&self, param: &EditParam) -> Result<Edit> {
        self.post::<EditParam, Edit>("edits", Some(param)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_create() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = EditParam::new(Models::TextDavinciEdit001, "Fix the spelling mistakes")
            .input("Can u actuqli fix spilling mistikes?")
            .temperature(0.5);
        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert!(resp.object().is_some());
        assert!(resp.error().is_none());
        Ok(())
    }
}
