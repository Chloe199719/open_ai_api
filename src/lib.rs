#![warn(
    clippy::all,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
#[cfg(feature = "chat")]
pub mod chat_completion;
#[cfg(feature = "models")]
pub mod models;
pub mod structs;

#[cfg(feature = "client")]
pub struct Client {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: String::new(),
            base_url: "https://api.openai.com".to_string(),
        }
    }
}

impl Client {
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self::with_base_url(api_key, None)
    }
    #[must_use]
    pub fn with_base_url(api_key: &str, base_url: Option<&str>) -> Self {
        match base_url {
            Some(base_url) => Self {
                client: reqwest::Client::new(),
                api_key: api_key.to_string(),
                base_url: base_url.to_string(),
            },
            None => Self {
                client: reqwest::Client::new(),
                api_key: api_key.to_string(),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests_modules {
    use super::*;

    #[test]
    fn crate_client() {
        let result = Client::new("api_key");
        assert!(result.api_key == "api_key");
    }
    #[test]
    fn crate_client_with_base_url() {
        let result = Client::with_base_url("api_key", Some("https://api.openai.com"));
        assert!(result.api_key == "api_key");
        assert!(result.base_url == "https://api.openai.com");
    }
}
