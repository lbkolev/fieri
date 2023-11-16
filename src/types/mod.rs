#![doc = include_str!("../../docs/types.md")]

use std::{
    fmt::Display,
    fs,
    io::{copy, Cursor},
    path::Path,
    str::FromStr,
};

use clap::{builder, Parser};
use derive_builder::Builder;
use reqwest::{
    get,
    multipart::{Form, Part},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;

use crate::{utils::is_false, Client, Result};

/// Possible Errors returned by responses from OpenAI.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestError {
    pub error: ErrorMessage,
}

#[derive(Clone, Debug, std::default::Default, Deserialize, Serialize)]
pub struct ErrorMessage {
    pub message: String,
    pub r#type: String,

    // those are most frequently returned as null from OpenAI, even in the occurence of an error.
    pub param: serde_json::Value,
    pub code: serde_json::Value,
}

/// Tokens used for the requested action from OpenAI.
#[derive(Clone, Debug, std::default::Default, Deserialize, Serialize)]
#[serde(default)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Clone, Debug, std::default::Default, Deserialize, Serialize)]
pub struct Choices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<f32>,
}

/// Information from requests wishing for a resource to be deleted, like [`Delete File`](crate::file::delete) and [`Delete Fine-tune`](crate::fine_tune::delete).
#[derive(Debug, std::default::Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Delete {
    pub id: String,
    pub object: String,
    pub deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Response from endpoints like [`Upload File`](crate::file::upload), [`Retrieve file`][crate::file::retrieve] & [`Create Fine-tune`](crate::fine_tune::create).
#[derive(Debug, std::default::Default, Deserialize, Serialize)]
#[serde(default)]
pub struct File {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

type Files = Vec<File>;

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize, Parser)]
#[builder(default, setter(into, strip_option))]
pub struct ChatParam {
    /// A list of messages describing the conversation so far.
    #[clap(short, long, required = true, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub messages: Vec<ChatMessage>,

    /// ID of the model to use.
    #[clap(long, default_value = "gpt-3.5-turbo")]
    pub model: String,

    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub frequency_penalty: Option<f32>,

    /// The maximum number of tokens to generate in the chat completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub max_tokens: Option<u32>,

    /// How many chat completion choices to generate for each input message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub n: Option<u32>,

    /// Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub presence_penalty: Option<f32>,

    /// This feature is in Beta.
    ///
    /// If specified, our system will make a best effort to sample deterministically,
    /// such that repeated requests with the same seed and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub seed: Option<u64>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub stop: Option<String>,

    /// If set, partial message deltas will be sent, like in ChatGPT.
    #[serde(skip_serializing_if = "is_false")]
    #[clap(long)]
    pub stream: bool,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub top_p: Option<f32>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[clap(long)]
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct ChatMessage {
    /// The role of the author of this message. One of system, user, or assistant.
    pub role: ChatRole,

    /// The contents of the message.
    pub content: String,

    /// The name of the author of this message. May contain a-z, A-Z, 0-9, and underscores, with a maximum length of 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Function,
}

impl Default for ChatRole {
    fn default() -> Self {
        Self::User
    }
}

impl From<String> for ChatRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "system" => Self::System,
            "user" => Self::User,
            "assistant" => Self::Assistant,
            "function" => Self::Function,
            _ => Self::User,
        }
    }
}

impl Display for ChatRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ChatRole::System => "system",
            ChatRole::User => "user",
            ChatRole::Assistant => "assistant",
            ChatRole::Function => "function",
        };
        write!(f, "{}", s)
    }
}

impl Serialize for ChatRole {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for ChatRole {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

impl ChatMessageBuilder {
    pub fn new(role: impl Into<ChatRole>, content: impl Into<String>) -> Self {
        Self {
            role: Some(role.into()),
            content: Some(content.into()),
            ..Self::default()
        }
    }
}

impl From<String> for ChatMessage {
    fn from(s: String) -> Self {
        Self {
            role: ChatRole::default(),
            content: s,
            name: Some("rand".to_string()),
        }
    }
}

impl ChatParamBuilder {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: Some(model.into()),
            messages: Some(messages),
            ..Self::default()
        }
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Chat {
    id: String,
    object: String,
    created: i64,
    pub choices: Vec<ChatChoice>,

    pub usage: TokenUsage,
    //#[serde(flatten)]
    pub error: Option<ErrorMessage>,
}

/// Parameters for [`Create Completion`](create) request.
#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct CompletionParam {
    /// The model to use for the completion request.
    model: String,

    /// The prompt(s) to generate completions for.
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<Vec<String>>,

    /// The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,

    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,

    /// What sampling temperature to use, between 0 and 2. Higher values means the model will take more risks.
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
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,

    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<f32>,

    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "is_false")]
    echo: bool,

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

/// Parameters for [`Create Embedding`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct EmbeddingParam {
    /// The model to use for the embedding request.
    model: String,

    /// Input text to get embeddings for, encoded as a string.
    ///
    /// Each input must not exceed 8192 tokens in length.
    input: String,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    user: Option<String>,
}

impl EmbeddingParamBuilder {
    pub fn new(model: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            model: Some(model.into()),
            input: Some(input.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create Embedding`](create) request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Embedding {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub mode: String,

    pub usage: Option<TokenUsage>,
}

/// The distance between two vectors measures their relatedness. Small distances suggest high relatedness and large distances suggest low relatedness.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Embeddings,
    pub index: u64,
}

type Embeddings = Vec<f32>;

/// Response from [`List File`](list) request.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ListFiles {
    pub data: Files,
    pub object: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// The Possible Purposes of the uploaded documents.
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Purpose {
    #[default]
    FineTune,
    Answers,
    Search,
    Classifications,
}

impl std::fmt::Display for Purpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Purpose::FineTune => write!(f, "fine-tune"),
            Purpose::Answers => write!(f, "answers"),
            Purpose::Search => write!(f, "search"),
            Purpose::Classifications => write!(f, "classifications"),
        }
    }
}

/// Parameters for [`Create Fine-tune`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct CreateFineTuneParam {
    /// The ID of an uploaded file that contains training data.
    ///
    /// See [upload](crate::file::upload) file for how to upload a file.
    training_file: String,

    /// The ID of an uploaded file that contains validation data.
    ///
    /// If you provide this file, the data is used to generate validation metrics periodically during fine-tuning. These metrics can be viewed in the fine-tuning results file.
    /// Your train and validation data should be mutually exclusive.
    // Note: Even though it's given as "optional" in the docs, it's required in the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_file: Option<String>,

    /// The name of the base model to fine-tune. You can select one of "ada", "babbage", "curie", "davinci", or a fine-tuned model created after 2022-04-21.
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    /// The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    n_epochs: Option<i32>,

    /// The batch size to use for training. The batch size is the number of training examples used to train a single forward and backward pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<i32>,

    /// The learning rate multiplier to use for training. The fine-tuning learning rate is the original learning rate used for pretraining multiplied by this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    learning_rate_multiplier: Option<f32>,

    /// The weight to use for loss on the prompt tokens.
    ///
    /// This controls how much the model tries to learn to generate the prompt (as compared to the completion which always has a weight of 1.0), and can add a stabilizing effect to training when completions are short.
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_loss_weight: Option<f32>,

    /// If set, we calculate classification-specific metrics such as accuracy and F-1 score using the validation set at the end of every epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_classification_metrics: Option<bool>,

    /// The number of classes in a classification task.
    ///
    /// This parameter is required for multiclass classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_n_classes: Option<i32>,

    /// The positive class in binary classification.
    ///
    /// This parameter is needed to generate precision, recall, and F1 metrics when doing binary classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_positive_class: Option<String>,

    /// If this is provided, we calculate F-beta scores at the specified beta values. The F-beta score is a generalization of F-1 score. This is only used for binary classification.
    ///
    /// With a beta of 1 (i.e. the F-1 score), precision and recall are given the same weight. A larger beta score puts more weight on recall and less on precision. A smaller beta score puts more weight on precision and less on recall.
    #[serde(skip_serializing_if = "Option::is_none")]
    classification_betas: Option<Vec<f32>>,

    /// Suffix to be added to the model's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
}

impl CreateFineTuneParamBuilder {
    pub fn new(training_file: impl Into<String>) -> Self {
        Self {
            training_file: Some(training_file.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create Fine-Tune`][create] request.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct FineTune {
    pub id: String,
    pub object: String,
    pub model: String,
    pub created_at: u64,
    pub events: Events,

    pub hyperparams: HyperParams,
    pub organization_id: String,
    pub result_files: Files,
    pub validation_files: Files,
    pub training_files: Files,
    pub status: String,
    pub updated_at: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Hyper parameters for fine-tuning a model.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct HyperParams {
    pub n_epochs: u32,
    pub batch_size: u32,
    pub learning_rate_multiplier: f32,
    pub prompt_loss_weight: f32,
    pub compute_classification_metrics: bool,
    pub classification_n_classes: u32,
    pub classification_positive_class: String,
    pub classification_betas: Vec<f32>,
}

/// Events occuring on Fine-tunes
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Event {
    pub object: String,
    pub created_at: u64,
    pub level: String,
    pub message: String,
}

type Events = Vec<Event>;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ListEvents {
    pub object: String,
    pub data: Vec<Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ListFineTune {
    pub object: String,
    pub data: Vec<FineTune>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// The size of the generated images.
///
/// Must be one of 256x256, 512x512, or 1024x1024.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum ImageSize {
    S256x256,
    S512x512,
    #[default]
    S1024x1024,
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageSize::S256x256 => write!(f, "256x256"),
            ImageSize::S512x512 => write!(f, "512x512"),
            ImageSize::S1024x1024 => write!(f, "1024x1024"),
        }
    }
}

impl FromStr for ImageSize {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "256x256" => Ok(ImageSize::S256x256),
            "512x512" => Ok(ImageSize::S512x512),
            "1024x1024" => Ok(ImageSize::S1024x1024),
            _ => Err(format!("Invalid ImageSize: {}", s)),
        }
    }
}

impl Serialize for ImageSize {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Parameters for [`Generate Image`](generate) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct GenerateImageParam {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u8>,

    /// The size of the generated images.
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<ImageSize>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl GenerateImageParamBuilder {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: Some(prompt.into()),
            ..Default::default()
        }
    }
}

/// Response from [Generate](generate), [Edit](edit) & [Variation](variate) requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Links>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

impl Image {
    /// Save the image(s) to the given directory.
    /// The images will be saved as based on the generated image id.
    ///
    /// For example, a generated image with url `https://oaidalleapiprodscus.blob.core.windows.net/private/org-123/user-456/img-789.png`
    /// Will be saved with a name of `img-789.png` in the given directory.
    ///
    ///
    /// ## Example
    /// ```no_run
    /// // Generate an image based on a prompt and save it locally.
    /// use fieri::{Client, image::{ImageSize, GenerateImageParamBuilder, generate}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///
    ///     let param = GenerateImageParamBuilder::new("A cat")
    ///         .size(ImageSize::S256x256)
    ///         .n(1)
    ///         .build()?;
    ///
    ///     let image = generate(&client, &param)
    ///         .await?
    ///         .save("/tmp/")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub async fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let Some(data) = &self.data {
            for (i, link) in data.iter().enumerate() {
                let resp = get(&link.url).await?;

                let def_img_name = format!("image_{i}.png");
                let fname = resp
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .unwrap_or(def_img_name.as_str());

                let full_path = Path::new(path.as_ref()).join(fname);
                let mut file = fs::File::create(full_path)?;
                let mut content = Cursor::new(resp.bytes().await?);
                copy(&mut content, &mut file)?;
            }
        }

        Ok(())
    }
}

/// link to an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub url: String,
}

type Links = Vec<Link>;

/// Parameters for [`Edit Image`](edit) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct EditImageParam {
    /// A text description of the desired image(s). The maximum length is 1000 characters.
    pub prompt: String,

    /// The number of images to generate. Must be between 1 and 10.
    pub n: u8,

    /// The size of the generated images.
    pub size: ImageSize,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub user: String,
}

impl Default for EditImageParam {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            n: 1,
            size: ImageSize::S1024x1024,
            user: String::new(),
        }
    }
}

impl EditImageParamBuilder {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: Some(prompt.into()),
            ..Default::default()
        }
    }
}

/// Parameters for [`Variate Image`](variate) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct VariateImageParam {
    /// The number of images to generate. Must be between 1 and 10.
    pub n: u8,

    /// The size of the generated images.
    pub size: ImageSize,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub user: String,
}

impl Default for VariateImageParam {
    fn default() -> Self {
        Self {
            n: 1,
            size: ImageSize::S1024x1024,
            user: String::new(),
        }
    }
}

impl VariateImageParamBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Response from [List Models](list) request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Models {
    pub data: Vec<Model>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Response from [Retrieve a Model](retrieve) request.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub permission: Vec<Permissions>,
    pub root: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_usage: Option<TokenUsage>,
}

/// Types of permissions that can be applied to a model.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Permissions {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub is_blocking: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// Parameters for [`Create Moderation`](create) request.
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Deserialize, Serialize)]
#[builder(default, setter(into, strip_option))]
pub struct ModerationParam {
    /// The content moderations model to use for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    /// The input text to classify.
    input: String,
}

impl ModerationParamBuilder {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: Some(input.into()),
            ..Self::default()
        }
    }
}

/// Response from [`Create Moderation`](create) request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Moderation {
    pub id: String,
    pub model: String,
    pub flagged: bool,
    pub results: Vec<ModerationResult>,

    pub token_usage: Option<TokenUsage>,
}

/// The result of the content moderation request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModerationResult {
    pub categories: Categories,
    pub category_scores: CategoryScores,
}

/// Contains a per-category binary content policy violation flags.
///
/// For each category, the value is `true` if the model flags the corresponding category as violated, `false` otherwise.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Categories {
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

/// Contains a per-category raw scores output by the model, denoting the model's confidence that the input violates the OpenAI's policy for the category.
///
/// The value is between 0 and 1, where higher values denote higher confidence.
///
/// The scores should not be interpreted as probabilities.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct CategoryScores {
    pub hate: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    pub violence: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_deserialization() {
        let param: ChatParam = serde_json::from_str(
            r#"
            {
                "model": "gpt-3.5-turbo",
                "messages": [{"role": "user", "content": "Hello!"}]
            }
            "#,
        )
        .unwrap();

        let resp: Chat = serde_json::from_str(
            r#"
            {
                "id": "chatcmpl-123",
                "object": "chat.completion",
                "created": 1677652288,
                "choices": [{
                  "index": 0,
                  "message": {
                    "role": "assistant",
                    "content": "\n\nHello there, how may I assist you today?"
                  },
                  "finish_reason": "stop"
                }],
                "usage": {
                  "prompt_tokens": 9,
                  "completion_tokens": 12,
                  "total_tokens": 21
                }
              }
            "#,
        )
        .unwrap();

        assert_eq!(param.model, "gpt-3.5-turbo");
        assert_eq!(param.messages.len(), 1);
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(
            resp.choices[0].message.content,
            "\n\nHello there, how may I assist you today?"
        );
        assert_eq!(resp.choices[0].finish_reason, Some("stop".to_string()));
        assert_eq!(resp.usage.prompt_tokens, 9);
    }

    #[test]
    fn test_create_completion_deserialization() {
        let param: CompletionParam = serde_json::from_str(
            r#"
            {
                "model": "text-davinci-003",
                "prompt": ["Say this is a test"],
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
        assert_eq!(param.prompt.unwrap(), vec!["Say this is a test"]);
        assert_eq!(param.suffix, None);
        assert_eq!(resp.choices.len(), 1);
        assert_eq!(
            resp.choices[0].text,
            Some("\n\nThis is indeed a test".to_string())
        );
        assert_eq!(resp.choices[0].logprobs, None);
        assert_eq!(resp.usage.unwrap().prompt_tokens, 5);
    }

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

    #[test]
    fn test_create_embedding_deserialization() {
        let param: EmbeddingParam = serde_json::from_str(
            r#"
            {
                "model": "text-embedding-ada-002",
                "input": "The food was delicious and the waiter..."
            }
            "#,
        )
        .unwrap();

        let resp: Embedding = serde_json::from_str(
            r#"
            {
                "object": "list",
                "data": [
                  {
                    "object": "embedding",
                    "embedding": [
                      0.0023064255,
                      -0.009327292,
                      -0.0028842222
                    ],
                    "index": 0
                  }
                ],
                "model": "text-embedding-ada-002",
                "usage": {
                  "prompt_tokens": 8,
                  "total_tokens": 8
                }
              }
            "#,
        )
        .unwrap();

        assert_eq!(param.model, "text-embedding-ada-002");
        assert_eq!(param.user, None);
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].embedding.len(), 3);
    }

    #[test]
    fn test_list_files_deserialization() {
        let resp: ListFiles = serde_json::from_str(
            r#"
            {
                "data": [
                  {
                    "id": "file-ccdDZrC3iZVNiQVeEA6Z66wf",
                    "object": "file",
                    "bytes": 175,
                    "created_at": 1613677385,
                    "filename": "train.jsonl",
                    "purpose": "search"
                  },
                  {
                    "id": "file-XjGxS3KTG0uNmNOK362iJua3",
                    "object": "file",
                    "bytes": 140,
                    "created_at": 1613779121,
                    "filename": "puppy.jsonl",
                    "purpose": "search"
                  }
                ],
                "object": "list"
              }
            "#,
        )
        .unwrap();

        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].id, "file-ccdDZrC3iZVNiQVeEA6Z66wf");
        assert_eq!(resp.data[1].object, "file");
    }

    #[test]
    fn test_upload_file_deserialization() {
        let resp: File = serde_json::from_str(
            r#"
            {
                "id": "file-XjGxS3KTG0uNmNOK362iJua3",
                "object": "file",
                "bytes": 140,
                "created_at": 1613779121,
                "filename": "mydata.jsonl",
                "purpose": "fine-tune"
              }
            "#,
        )
        .unwrap();

        assert_eq!(resp.id, "file-XjGxS3KTG0uNmNOK362iJua3");
        assert_eq!(resp.object, "file");
    }

    #[test]
    fn test_create_fine_tune_deserialization() {
        let resp: FineTune = serde_json::from_str(
            r#"
            {
                "id": "ft-AF1WoRqd3aJAHsqc9NY7iL8F",
                "object": "fine-tune",
                "model": "curie",
                "created_at": 1614807352,
                "events": [
                  {
                    "object": "fine-tune-event",
                    "created_at": 1614807352,
                    "level": "info",
                    "message": "Job enqueued. Waiting for jobs ahead to complete. Queue number: 0."
                  }
                ],
                "fine_tuned_model": null,
                "hyperparams": {
                  "batch_size": 4,
                  "learning_rate_multiplier": 0.1,
                  "n_epochs": 4,
                  "prompt_loss_weight": 0.1
                },
                "organization_id": "org-...",
                "result_files": [],
                "status": "pending",
                "validation_files": [],
                "training_files": [
                  {
                    "id": "file-XGinujblHPwGLSztz8cPS8XY",
                    "object": "file",
                    "bytes": 1547276,
                    "created_at": 1610062281,
                    "filename": "my-data-train.jsonl",
                    "purpose": "fine-tune-train"
                  }
                ],
                "updated_at": 1614807352
            }
            "#,
        )
        .unwrap();

        assert_eq!(resp.id, "ft-AF1WoRqd3aJAHsqc9NY7iL8F");
        assert_eq!(resp.object, "fine-tune");
        assert_eq!(resp.events.len(), 1);
        assert_eq!(resp.training_files[0].filename, "my-data-train.jsonl");
    }

    #[test]
    fn test_list_fine_tune_events_deserialization() {
        let resp: ListEvents = serde_json::from_str(
            r#"
            {
                "object": "list",
                "data": [
                  {
                    "object": "fine-tune-event",
                    "created_at": 1614807352,
                    "level": "info",
                    "message": "Job enqueued. Waiting for jobs ahead to complete. Queue number: 0."
                  },
                  {
                    "object": "fine-tune-event",
                    "created_at": 1614807356,
                    "level": "info",
                    "message": "Job started."
                  },
                  {
                    "object": "fine-tune-event",
                    "created_at": 1614807861,
                    "level": "info",
                    "message": "Uploaded snapshot: curie:ft-acmeco-2021-03-03-21-44-20."
                  },
                  {
                    "object": "fine-tune-event",
                    "created_at": 1614807864,
                    "level": "info",
                    "message": "Uploaded result files: file-QQm6ZpqdNwAaVC3aSz5sWwLT."
                  },
                  {
                    "object": "fine-tune-event",
                    "created_at": 1614807864,
                    "level": "info",
                    "message": "Job succeeded."
                  }
                ]
              }
            "#,
        )
        .unwrap();

        assert_eq!(resp.data.len(), 5);
        assert_eq!(resp.data[0].level, "info");
    }

    #[test]
    fn test_delete_fine_tune_deserialization() {
        let resp: Delete = serde_json::from_str(
            r#"
            {
                "id": "curie:ft-acmeco-2021-03-03-21-44-20",
                "object": "model",
                "deleted": true
            }
            "#,
        )
        .unwrap();

        assert_eq!(resp.id, "curie:ft-acmeco-2021-03-03-21-44-20");
    }

    #[test]
    fn test_parse_image_response_deserialization() {
        let param: GenerateImageParam = serde_json::from_str(
            r#"{
                "prompt": "A cute baby sea otter",
                "size": "S256x256",
                "n": 1
            }"#,
        )
        .unwrap();

        let result: Image = serde_json::from_str(
            r#"
            {
                "created": 1589478378,
                "data": [
                    {
                        "url": "https://..."
                    },
                    {
                        "url": "https://..."
                    }
                ]
            }
        "#,
        )
        .unwrap();

        assert_eq!(param.prompt, "A cute baby sea otter");
        assert_eq!(param.size, Some(ImageSize::S256x256));
        assert_eq!(param.user, None);
        assert_eq!(result.data.unwrap().len(), 2);
    }

    #[test]
    fn test_model_list_deserialization() {
        let resp: Models = serde_json::from_str(
            r#"
            {
                "data": [
                  {
                    "id": "model-id-0",
                    "object": "model",
                    "created": 1623155849,
                    "owned_by": "organization-owner",
                    "permission": [],
                    "root": "model-id-1"
                  },
                  {
                    "id": "model-id-1",
                    "object": "model",
                    "created": 11,
                    "owned_by": "organization-owner",
                    "permission": [],
                    "root": "model-id-0"
                  },
                  {
                    "id": "model-id-2",
                    "object": "model",
                    "created": 1234567890,
                    "owned_by": "openai",
                    "permission": [],
                    "root": "model-id-2"
                  }
                ],
                "object": "list"
              }
            "#,
        )
        .unwrap();

        assert_eq!(resp.data.len(), 3);
        assert_eq!(resp.data[0].id, "model-id-0");
        assert!(resp.token_usage.is_none());
    }

    #[test]
    fn test_model_retrieve_deserialization() {
        let resp: Model = serde_json::from_str(
            r#"
            {
                "id": "text-davinci-003",
                "object": "model",
                "created": 1623155849,
                "owned_by": "openai",
                "permission": [],
                "root": "text-davinci-003"
              }
            "#,
        )
        .unwrap();

        assert_eq!(resp.id, "text-davinci-003");
        assert!(resp.token_usage.is_none());
    }

    #[test]
    fn test_create_moderation_deserialization() {
        let param: ModerationParam = serde_json::from_str(
            r#"
            {
                "input": "I want to kill them."
              }
            "#,
        )
        .unwrap();

        let resp: Moderation = serde_json::from_str(
            r#"
            {
                "id": "modr-5MWoLO",
                "model": "text-moderation-001",
                "results": [
                  {
                    "categories": {
                      "hate": false,
                      "hate/threatening": true,
                      "self-harm": false,
                      "sexual": false,
                      "sexual/minors": false,
                      "violence": true,
                      "violence/graphic": false
                    },
                    "category_scores": {
                      "hate": 0.22714105248451233,
                      "hate/threatening": 0.4132447838783264,
                      "self-harm": 0.005232391878962517,
                      "sexual": 0.01407341007143259,
                      "sexual/minors": 0.0038522258400917053,
                      "violence": 0.9223177433013916,
                      "violence/graphic": 0.036865197122097015
                    },
                    "flagged": true
                  }
                ]
              }
            "#,
        )
        .unwrap();

        assert_eq!(param.input, "I want to kill them.");
        assert_eq!(resp.id, "modr-5MWoLO");
        assert_eq!(resp.model, "text-moderation-001");
        assert_eq!(resp.results.len(), 1);
    }
}
