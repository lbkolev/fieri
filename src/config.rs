//! Configuration for the OpenAI API.

use derive_getters::Getters;
use serde::{Serialize, Serializer};

/// The configuration needed to establish connection with OpenAI's API.
#[derive(Debug, Clone, Getters)]
pub struct Config {
    api_key: String,

    url: String,

    /// For users who belong to multiple organizations, you can pass a header
    /// to specify which organization is used for an API request.
    #[getter(skip)]
    pub organization: Option<String>,

    #[getter(skip)]
    pub default_model: Option<Models>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            organization: None,
            default_model: None,
        }
    }
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Self::default()
        }
    }

    pub fn organization(mut self, organization: Option<String>) -> Self {
        self.organization = organization;

        self
    }

    /// Optional default model to use for requests.
    pub fn default_model(mut self, model: Option<Models>) -> Self {
        self.default_model = model;

        self
    }
}

/// All the available Models offered for usage through the API.
///
/// Extracted from [Models List].
///
/// [Models List]: crate::api_resources::model::list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Models {
    None,

    Ada,
    AdaCodeSearchCode,
    AdaCodeSearchText,
    AdaSearchDocument,
    AdaSearchQuery,
    AdaSimilarity,
    Ada20200503,

    AudioTranscribe001,

    Babbage,
    BabbageCodeSearchCode,
    BabbageCodeSearchText,
    BabbageSearchDocument,
    BabbageSearchQuery,
    BabbageSimilarity,
    Babbage20200503,

    CodeCushman001,
    CodeDavinci002,
    CodeDavinciEdit001,
    CodeSearchAdaCode001,
    CodeSearchAdaText001,
    CodeSearchBabbageCode001,
    CodeSearchBabbageText001,

    Curie,
    CurieInstructBeta,
    CurieSearchDocument,
    CurieSearchQuery,
    CurieSimilarity,
    Curie20200503,

    Cushman20200503,

    Davinci,
    DavinciIf300,
    DavinciInstructBeta,
    DavinciInstructBeta200,
    DavinciSearchDocument,
    DavinciSearchQuery,
    DavinciSimilarity,
    Davinci20200503,

    IfCuriev2,
    IfDavinciv2,
    IfDavinci300,

    TextAda001,
    TextBabbage001,
    TextCurie001,
    TextDavinci001,
    TextDavinci002,
    TextDavinci003,
    TextDavinciEdit001,
    TextDavinciInsert001,
    TextDavinciInsert002,
    TextEmbeddingAda002,
    TextSearchAdaDoc001,
    TextSearchAdaQuery001,
    TextSearchBabbageDoc001,
    TextSearchBabbageQuery001,
    TextSearchCurieDoc001,
    TextSearchCurieQuery001,
    TextSearchDavinciDoc001,
    TextSearchDavinciQuery001,
    TextSimilarityAda001,
    TextSimilarityBabbage001,
    TextSimilarityCurie001,
    TextSimilarityDavinci001,
}

impl std::fmt::Display for Models {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Models::*;

        match self {
            None => write!(f, "none"),
            Ada => write!(f, "ada"),
            AdaCodeSearchCode => write!(f, "ada-code-search-code"),
            AdaCodeSearchText => write!(f, "ada-code-search-text"),
            AdaSearchDocument => write!(f, "ada-search-document"),
            AdaSearchQuery => write!(f, "ada-search-query"),
            AdaSimilarity => write!(f, "ada-similarity"),
            Ada20200503 => write!(f, "ada:2020-05-03"),

            AudioTranscribe001 => write!(f, "audio-transcribe-001"),

            Babbage => write!(f, "babbage"),
            BabbageCodeSearchCode => write!(f, "babbage-code-search-code"),
            BabbageCodeSearchText => write!(f, "babbage-code-search-text"),
            BabbageSearchDocument => write!(f, "babbage-search-document"),
            BabbageSearchQuery => write!(f, "babbage-search-query"),
            BabbageSimilarity => write!(f, "babbage-similarity"),
            Babbage20200503 => write!(f, "babbage:2020-05-03"),

            CodeCushman001 => write!(f, "code-cushman-001"),
            CodeDavinci002 => write!(f, "code-davinci-002"),
            CodeDavinciEdit001 => write!(f, "code-davinci-edit-001"),
            CodeSearchAdaCode001 => write!(f, "code-search-ada-code-001"),
            CodeSearchAdaText001 => write!(f, "code-search-ada-text-001"),
            CodeSearchBabbageCode001 => write!(f, "code-search-babbage-code-001"),
            CodeSearchBabbageText001 => write!(f, "code-search-babbage-text-001"),

            Curie => write!(f, "curie"),
            CurieInstructBeta => write!(f, "curie-instruct-beta"),
            CurieSearchDocument => write!(f, "curie-search-document"),
            CurieSearchQuery => write!(f, "curie-search-query"),
            CurieSimilarity => write!(f, "curie-similarity"),
            Curie20200503 => write!(f, "curie:2020-05-03"),

            Cushman20200503 => write!(f, "cushman:2020-05-03"),

            Davinci => write!(f, "davinci"),
            DavinciIf300 => write!(f, "davinci-if-300"),
            DavinciInstructBeta => write!(f, "davinci-instruct-beta"),
            DavinciInstructBeta200 => write!(f, "davinci-instruct-beta:2.0.0"),
            DavinciSearchDocument => write!(f, "davinci-search-document"),
            DavinciSearchQuery => write!(f, "davinci-search-query"),
            DavinciSimilarity => write!(f, "davinci-similarity"),
            Davinci20200503 => write!(f, "davinci:2020-05-03"),

            IfCuriev2 => write!(f, "if-curie-v2"),
            IfDavinciv2 => write!(f, "if-davinci-v2"),
            IfDavinci300 => write!(f, "if-davinci:3.0.0"),

            TextAda001 => write!(f, "text-ada-001"),
            TextBabbage001 => write!(f, "text-babbage-001"),
            TextCurie001 => write!(f, "text-curie-001"),
            TextDavinci001 => write!(f, "text-davinci-001"),
            TextDavinci002 => write!(f, "text-davinci-002"),
            TextDavinci003 => write!(f, "text-davinci-003"),
            TextDavinciEdit001 => write!(f, "text-davinci-edit-001"),
            TextDavinciInsert001 => write!(f, "text-davinci-insert-001"),
            TextDavinciInsert002 => write!(f, "text-davinci-insert-002"),
            TextEmbeddingAda002 => write!(f, "text-embedding-ada-002"),
            TextSearchAdaDoc001 => write!(f, "text-search-ada-doc-001"),
            TextSearchAdaQuery001 => write!(f, "text-search-ada-query-001"),
            TextSearchBabbageDoc001 => write!(f, "text-search-babbage-doc-001"),
            TextSearchBabbageQuery001 => write!(f, "text-search-babbage-query-001"),
            TextSearchCurieDoc001 => write!(f, "text-search-curie-doc-001"),
            TextSearchCurieQuery001 => write!(f, "text-search-curie-query-001"),
            TextSearchDavinciDoc001 => write!(f, "text-search-davinci-doc-001"),
            TextSearchDavinciQuery001 => write!(f, "text-search-davinci-query-001"),
            TextSimilarityAda001 => write!(f, "text-similarity-ada-001"),
            TextSimilarityBabbage001 => write!(f, "text-similarity-babbage-001"),
            TextSimilarityCurie001 => write!(f, "text-similarity-curie-001"),
            TextSimilarityDavinci001 => write!(f, "text-similarity-davinci-001"),
        }
    }
}

impl Serialize for Models {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
