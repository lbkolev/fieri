//! The Client used to establish a connection and interact with the OpenAI API.
//!
//!
//! ## Usage
//! ```no_run
//! use fieri::Client;
//!
//! let client = Client::new();
//! ```
//!
//! ## Usage with explicity specified API Key and [Organization](https://beta.openai.com/docs/api-reference/requesting-organization)
//! ```no_run
//! use fieri::Client;
//!
//! let client = Client::new()
//!     .api_key("...")
//!     .organization("...");
//! ```

use std::fmt::Debug;

use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    multipart,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{config::Config, error::Error, types::RequestError, Result};

// Response returned by each interaction with OpenAI, either an error or a valid generic.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Invalid(RequestError),
    Valid(T),
}

/// The Client used to interact with the OpenAI API.
#[derive(Clone, Debug, Default)]
pub struct Client {
    /// Configuration needed to authorize against the API.
    config: Config,

    /// The HTTP client that'll execute requests.
    handler: reqwest::Client,
}

impl Client {
    /// Creates a new instance of the Client.
    /// The API key is read from the `OPENAI_API_KEY` environment variable.
    /// The API Organization is read from the `OPENAI_ORGANIZATION` environment variable.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();

        let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "".to_string());
        let organization = std::env::var("OPENAI_ORGANIZATION").unwrap_or_else(|_| "".to_string());

        if !api_key.is_empty() {
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {api_key}")
                    .parse()
                    .expect("Unable to parse the API key."),
            );
        }

        if !organization.is_empty() {
            headers.insert(
                "OpenAI-Organization",
                organization
                    .parse()
                    .expect("Unable to parse the given Organization."),
            );
        }

        let config = Config::new(api_key).headers(headers.clone());
        Self {
            config,
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating request handler."),
        }
    }

    // Used by the unit/integr tests.
    pub(crate) fn mock_new(url: Url) -> Self {
        let config = Config::mock_new(url);
        Self {
            config,
            handler: reqwest::Client::new(),
        }
    }

    /// Explicitly specify the api key.
    /// By default, the api key is read from the `OPENAI_API_KEY` environment variable.
    /// If both `OPENAI_API_KEY` and `api_key` are set, the `api_key` takes precedence.
    pub fn api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        let api_key = api_key.into();
        let mut headers = self.config.headers;
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {api_key}")
                .parse()
                .expect("Unable to parse the API key."),
        );

        self.config.api_key = api_key;
        self.config.headers = headers.clone();

        Self {
            config: self.config,
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating request handler."),
        }
    }

    /// For users who belong to multiple organizations, you can pass a header
    /// to specify which organization is used for an API request.
    /// By default, the organization is read from the `OPENAI_ORGANIZATION` environment variable.
    /// If both `OPENAI_ORGANIZATION` and `organization` are set, the `organization` takes precedence.
    pub fn organization<T: Into<String>>(mut self, organization: T) -> Self {
        let organization = organization.into();
        let mut headers = self.config.headers;
        headers.insert(
            "OpenAI-Organization",
            organization
                .parse()
                .expect("Unable to parse the given Organization."),
        );

        self.config.organization = organization;
        self.config.headers = headers.clone();

        Self {
            config: self.config,
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating a request handler."),
        }
    }

    pub async fn get<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .get(self.config.url.join(identifier)?)
            .query(&param)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }

    pub async fn get_stream<X>(
        &self,
        identifier: &str,
        param: Option<&X>,
    ) -> Result<reqwest::Response>
    where
        X: Serialize,
    {
        let resp = self
            .handler
            .get(self.config.url.join(identifier)?)
            .query(&param)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn post<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(self.config.url.join(identifier)?)
            .json(&param)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }

    pub async fn post_stream<X>(
        &self,
        identifier: &str,
        param: Option<&X>,
    ) -> Result<reqwest::Response>
    where
        X: Serialize,
    {
        let resp = self
            .handler
            .post(self.config.url.join(identifier)?)
            .json(&param)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn post_data<Y>(&self, identifier: &str, data: multipart::Form) -> Result<Y>
    where
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(self.config.url.join(identifier)?)
            .multipart(data)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }

    pub async fn delete<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .delete(self.config.url.join(identifier)?)
            .query(&param)
            .send()
            .await?
            .json::<Response<Y>>()
            .await?;

        match resp {
            Response::Invalid(resp) => Err(Error::APIError(resp)),
            Response::Valid(resp) => Ok(resp),
        }
    }
}
