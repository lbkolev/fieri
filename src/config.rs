use derive_getters::Getters;

/// The configuration needed to establish connection with OpenAI's API.
#[derive(Debug, Getters)]
pub struct Config {
    api_key: String,

    url: String,

    /// For users who belong to multiple organizations, you can pass a header
    /// to specify which organization is used for an API request.
    organization: Option<String>,
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self {
            url: "https://api.openai.com/v1".to_string(),
            api_key,
            organization: None,
        }
    }

    pub fn add_organization(mut self, organization: String) -> Self {
        self.organization = Some(organization);

        self
    }
}
