//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.

use derive_getters::Getters;
use serde::Deserialize;

use crate::{api_resources::model::Model, client::Client, Result};

/// TMP
#[derive(Debug, Getters)]
pub struct CompletionParam {
    /// TODO: make a link to the models::list_models function
    /// ID of the model to use. You can use the List models API to see all of your available models.
    pub model: Model,

    /// The prompt(s) to generate completions for, encoded as a string.
    pub prompt: Option<String>,

    /// The suffix that comes after a completion of inserted text.
    pub suffix: Option<String>,

    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    pub max_tokens: Option<u16>,

    /// Higher values means the model will take more risks. 
    /// 
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    pub temperature: Option<f32>,

    pub top_p: Option<f32>,
}

//impl Default for CompletionParam {
    // fn default() -> Self {
    //    Self { model: Model::None }
    //}
//}
