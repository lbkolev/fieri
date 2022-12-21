//! Given a prompt and an instruction, the model will return an edited version of the prompt.

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{client::Client, Result, api_resources::model::Model, api_resources::completion::TokenUsage};


#[derive(Debug, Getters, Serialize)]
pub struct EditParam {
    model: String, 

    input: String,

    instruction: String,

    n: u32,

    temperature: f32,

    top_p: f32,
}

impl Default for EditParam {
    fn default() -> Self {
        Self {
            model: Model::None.to_string(),
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

    pub fn add_model(mut self, model: Model) -> Self {
        self.model = model.to_string();

        self
    }

    pub fn add_input(mut self, input: String) -> Self {
        self.input = input;

        self
    }

    pub fn add_instruction(mut self, instruction: String) -> Self {
        self.instruction = instruction;

        self
    }

    pub fn add_n(mut self, n: u32) -> Self {
        self.n = n;

        self
    }

    pub fn add_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;

        self
    }

    pub fn add_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;

        self
    }
}

#[derive(Debug, Getters, Deserialize)]
pub struct EditResp {
    object: Option<String>,
    created: Option<u64>,
    choices: Option<Vec<Choices>>,
    usage: Option<TokenUsage>,
}

#[derive(Debug, Getters, Deserialize)]
pub struct Choices {
    text: String,
    index: u32,
}

/// Creates a new edit for the provided input, instruction, and parameters
/// 
/// Example:
/// ```rust
/// use std::env;
/// use openai_rs::{
///     client::Client,
///     config::Config,
///     api_resources::{
///         model::Model,
///         edit::{
///             create_edit,
///             EditParam,
///             EditResp,
///         }
///     }
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new(env::var("OPENAI_API_KEY")?);
///     let client = Client::new(&config);
///
///     let param = EditParam::new()
///         .add_model(Model::TextDavinci001)
///         .add_input("What day of the wek is it?".to_string())
///         .add_instruction("Fix the spelling mistakes".to_string());
///     let resp: EditResp = create_edit(&client, param).await?;
///     println!("{:#?}", resp);
///     Ok(())
/// }
/// ```
pub async fn create_edit(client: &Client<'_>, param: EditParam) -> Result<EditResp> {
    client.create_edit(param).await
}

impl<'a> Client<'a> {
    async fn create_edit(&self, param: EditParam) -> Result<EditResp> {
        let resp = self
            .post::<EditParam, EditResp>("/edits".to_string(), Some(param))
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {}