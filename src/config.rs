//! Configuration for the OpenAI API.

use derive_getters::Getters;
use reqwest::header::HeaderMap;
use url::Url;

/// The configuration needed to establish connection with OpenAI's API.
#[derive(Debug, Clone, Getters)]
pub struct Config {
    api_key: String,

    url: Url,

    /// Headers used with each request.
    #[getter(skip)]
    pub headers: HeaderMap,

    #[getter(skip)]
    pub organization: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            url: Url::parse("https://api.openai.com/v1/").unwrap(),
            headers: HeaderMap::new(),
            organization: String::new(),
        }
    }
}

impl Config {
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: api_key.into(),
            ..Self::default()
        }
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;

        self
    }
}
