//! Given a prompt and an instruction, the model will return an edited version of the prompt.

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    api_resources::{Choices, ErrorResp, TokenUsage},
    client::Client,
    config::Models,
    error::Error,
    Result,
};

/// Parameters for [`create`](crate::api_resources::edit::create) edit request.
#[derive(Debug, Clone, Serialize)]
pub struct EditParam {
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

    pub fn instruction(mut self, instruction: String) -> Self {
        self.instruction = instruction;

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

/// Response from [`create`](crate::api_resources::edit::create) edit request.
#[derive(Debug, Getters, Deserialize)]
pub struct EditResp {
    object: Option<String>,
    created: Option<u64>,
    choices: Option<Vec<Choices>>,
    usage: Option<TokenUsage>,
    error: Option<ErrorResp>,
}

/// Creates a new edit for the provided input, instruction, and parameters.
///
/// Related OpenAI docs: [Create an Edit](https://beta.openai.com/docs/api-reference/edits/create)
///
/// ## Example
/// ```rust
/// use std::env;
/// use openai_rs::{
///     Models,
///     client::Client,
///     config::Config,
///     api_resources::edit::{
///         create,
///         EditParam,
///         EditResp,
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let param = EditParam::new()
///         .model(Some(Models::TextDavinci001))
///         .input("What dey of the wek is it?".to_string())
///         .instruction("Fix the spelling mistakes".to_string());
///     let resp: EditResp = create(&client, &param).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn create(client: &Client<'_>, param: &EditParam) -> Result<EditResp> {
    client.create_edit(param).await
}

impl<'a> Client<'a> {
    async fn create_edit(&self, param: &EditParam) -> Result<EditResp> {
        if param.model.is_none() && self.config().default_model.is_none() {
            return Err(Error::MissingModel);
        } else if param.model.is_some() {
            let resp = self
                .post::<EditParam, EditResp>("/edits".to_string(), Some(param))
                .await?;

            return Ok(resp);
        }

        let param = &EditParam {
            model: self.config().to_owned().default_model,
            ..param.to_owned()
        };
        let resp = self
            .post::<EditParam, EditResp>("/edits".to_string(), Some(param))
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
        // .default_model(Some(Model::TextDavinci001));
        let client = Client::new(&config);

        let param = EditParam::new()
            .model(Some(Models::TextDavinciEdit001))
            .input("Can u actuqli fix spilling mistekis?".to_string())
            .instruction("Fix the spelling mistakes".to_string())
            .temperature(0.5);

        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        Ok(())
    }
}
