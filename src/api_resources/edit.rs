//! Given a prompt and an instruction, the model will return an edited version of the prompt.
//!
//! The edits endpoint can be used to edit text, rather than just completing it. You provide some text and an instruction for how to modify it.
//!
//! This is a natural interface for translating, editing, and tweaking text. This is also useful for refactoring and working with code.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    api_resources::{Choices, TokenUsage},
    Client, Result,
};

/// Parameters for [`Create Edit`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct EditParam {
    /// The model to use for the edit request.
    model: String,

    /// The instruction that tells the model how to edit the prompt.
    instruction: String,

    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,

    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,

    /// What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// It's recommended to alter this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// It's recommended to alter this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Edit {
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choices>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
}

/// Creates a new edit for the provided input, instruction, and parameters.
///
/// Related OpenAI docs: [Create an Edit](https://beta.openai.com/docs/api-reference/edits/create)
///
/// ## Example
/// ```no_run
/// use fieri::{Client, edit::{create, EditParamBuilder}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = Client::new();
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

    #[test]
    fn test_create_edit_deserialization() {
        let param: EditParam = serde_json::from_str(
            r#"
            {
                "model": "text-davinci-edit-001",
                "input": "What day of the wek is it?",
                "instruction": "Fix the spelling mistakes"
            }
            "#,
        )
        .unwrap();

        let resp: Edit = serde_json::from_str(
            r#"
            {
                "object": "edit",
                "created": 1589478378,
                "choices": [
                    {
                        "text": "What day of the week is it?",
                        "index": 0
                    }
                ],
                "usage": {
                    "prompt_tokens": 25,
                    "completion_tokens": 32,
                    "total_tokens": 57
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(param.model, "text-davinci-edit-001");
        assert_eq!(param.n, None);
        assert_eq!(resp.object, "edit");
        assert_eq!(resp.choices.len(), 1);
    }
}
