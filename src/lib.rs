#[cfg(feature = "chat")]
pub mod chat_completion;
#[cfg(feature = "models")]
pub mod models;
pub mod structs;

#[cfg(feature = "client")]
pub struct Client {
    client: reqwest::Client,
    api_key: String,
}
#[cfg(feature = "client")]
impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
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
}
