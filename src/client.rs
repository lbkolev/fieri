use derive_getters::Getters;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    multipart,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Config, Result};

/// The Client used to interact with the OpenAI API.
#[derive(Debug, Getters)]
pub struct Client {
    /// Configuration needed to authorize against the API.
    config: Config,

    /// The HTTP client that'll execute requests.
    handler: reqwest::Client,
}

impl Client {
    pub fn new<T: Into<String> + std::fmt::Display>(api_key: T) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            format!("Bearer {api_key}")
                .parse()
                .expect("Unable to parse the API key."),
        );
        let config = Config::new(api_key).headers(headers.clone());

        Self {
            config,
            handler: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Err creating a request handler."),
        }
    }

    pub fn organization(mut self, organization: String) -> Self {
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
            .get(self.config().url().join(identifier)?)
            .query(&param)
            .send()
            .await?
            .json::<Y>()
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
            .post(self.config().url().join(identifier)?)
            .json(&param)
            .send()
            .await?
            .json::<Y>()
            .await?;

        Ok(resp)
    }

    pub async fn delete<X, Y>(&self, identifier: &str, param: Option<&X>) -> Result<Y>
    where
        X: Serialize,
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .delete(self.config().url().join(identifier)?)
            .query(&param)
            .send()
            .await?
            .json::<Y>()
            .await?;

        Ok(resp)
    }

    pub async fn post_data<X, Y>(&self, identifier: &str, param: multipart::Form) -> Result<Y>
    where
        Y: DeserializeOwned,
    {
        let resp = self
            .handler
            .post(self.config().url().join(identifier)?)
            .multipart(param)
            .send()
            .await?
            .json::<Y>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {}
