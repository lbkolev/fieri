//! Configuration for the OpenAI API.

use reqwest::header::HeaderMap;
use url::Url;

const DEFAULT_URL: &str = "https://api.openai.com/v1/";

/// The configuration needed to establish connection with OpenAI's API.
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,

    pub url: Url,

    /// Headers used with each request.
    pub headers: HeaderMap,

    pub organization: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            url: Url::parse(DEFAULT_URL).unwrap(),
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
