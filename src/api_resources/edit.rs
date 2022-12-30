//! Given a prompt and an instruction, the model will return an edited version of the prompt.
//!
//! The edits endpoint can be used to edit text, rather than just completing it. You provide some text and an instruction for how to modify it.
//!
//! This is a natural interface for translating, editing, and tweaking text. This is also useful for refactoring and working with code.

use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    api_resources::{Choices, RequestError, TokenUsage},
    Client, Result,
};

/// Parameters for [`Create Edit`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct EditParam {
    /// The model to use for the edit request.
    model: String,

    /// The instruction that tells the model how to edit the prompt.
    instruction: String,

    /// The input text to use as a starting point for the edit.
    input: Option<String>,

    /// How many edits to generate for the input and instruction.
    n: Option<u32>,

    /// What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// It's recommended to alter this or `top_p` but not both.
    temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It's recommended to alter this or `temperature` but not both.
    top_p: Option<f32>,
}

impl EditParamBuilder {
    pub fn new(model: impl Into<String>, instruction: impl Into<String>) -> Self {
        Self {
            model: Some(model.into()),
            instruction: Some(instruction.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create Edit`](create) request.
#[derive(Debug, Default, Deserialize, Getters)]
#[serde(default)]
pub struct Edit {
    object: String,
    created: u64,
    choices: Vec<Choices>,

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
/// use fieri::{Client, edit::{create, EditParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new(env::var("OPENAI_API_KEY")?);
///
///     let param = EditParamBuilder::new("text-davinci-edit-001", "Fix the spelling mistakes")
///         .input("What dey of the wek is it?")
///         .temperature(0.5)
///         .build()?;
///
///     let resp = create(&client, &param).await?;
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
    async fn test_create_edit() -> Result<()> {
        let client = Client::new(env::var("OPENAI_API_KEY")?);

        let param = EditParamBuilder::new("text-davinci-edit-001", "Fix the spelling mistakes")
            .input("What dey of the wek is it?")
            .temperature(0.5)
            .build()?;

        let resp = create(&client, &param).await?;
        println!("{:#?}", resp);

        assert_eq!(resp.object(), "edit");
        assert!(resp.usage().is_some());
        assert!(resp.error().is_none());

        Ok(())
    }
}
