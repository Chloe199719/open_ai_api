use crate::{
    structs::{
        errors::{ClientError, Error, ErrorData},
        model::{Model, ModelData},
    },
    Client,
};

impl Client {
    /// Get all models
    /// # Example
    /// ```
    /// use open_ai_api::Client;
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv::dotenv().ok();
    ///     let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());
    ///     let response = client.get_models().await;
    ///     match response {
    ///         Ok(model) => {
    ///             println!("{:#?}", model);
    ///         }
    ///         Err(e) => {
    ///             println!("{:#?}", e);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_models(&self) -> Result<Model, ClientError> {
        let response = self
            .client
            .get(format!("{}/v1/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if response.status().is_success() {
            let models = response.json::<Model>().await?;
            Ok(models)
        } else {
            let error = response.json::<Error>().await.unwrap_or_else(|_| Error {
                error: ErrorData {
                    param: None,
                    message: "Unknown".to_string(),
                    error_type: "Unknown".to_string(),
                    code: "Unknown".to_string(),
                },
            });
            Err(ClientError::ApiError(error))
        }
    }
    /// Get a model by id
    /// # Arguments
    /// * `id` - A string that holds the id of the model
    /// # Example
    /// ```
    /// use open_ai_api::Client;
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv::dotenv().ok();
    ///     let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());
    ///     let response = client.get_model_by_id("id").await;
    ///     match response {
    ///         Ok(model) => {
    ///             println!("{:#?}", model);
    ///         
    ///        }
    ///        Err(e) => {
    ///           println!("{:#?}", e);
    ///       }
    ///   }
    /// }
    /// ```

    pub async fn get_model_by_id(&self, id: &str) -> Result<ModelData, ClientError> {
        let response = self
            .client
            .get(&format!("https://api.openai.com/v1/models/{}", id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let model = response.json::<ModelData>().await?;
            Ok(model)
        } else {
            let error = response.json::<Error>().await.unwrap_or_else(|_| Error {
                error: ErrorData {
                    param: None,
                    message: "Unknown".to_string(),
                    error_type: "Unknown".to_string(),
                    code: "Unknown".to_string(),
                },
            });
            Err(ClientError::ApiError(error))
        }
    }
}

#[cfg(test)]
mod test_models_live {
    use crate::Client;

    #[ignore = "This test requires a valid API key"]
    #[tokio::test]
    async fn get_models() {
        dotenv::dotenv().ok();
        let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());
        let response = client.get_models().await.unwrap();
        assert!(response.object == "list");
    }

    #[tokio::test]
    async fn get_models_not_valid_api_key() {
        let client = Client::new("api_key");
        let response = client.get_models().await;
        assert!(response.is_err());
        if let Err(res) = response {
            match res {
                crate::structs::errors::ClientError::ApiError(e) => {
                    assert_eq!(e.error.error_type, "invalid_request_error");
                    assert_eq!(e.error.code, "invalid_api_key");
                }
                _ => panic!("Error type not expected"),
            }
        }
    }
    #[ignore = "This test requires a valid API key"]
    #[tokio::test]
    async fn get_models_one_model() {
        dotenv::dotenv().ok();
        let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());

        let response = client.get_models().await.unwrap();
        assert!(response.object == "list");
        let first_model = response.data.first().unwrap();
        let response_one_model = client.get_model_by_id(&first_model.id).await.unwrap();
        assert!(response_one_model.id == first_model.id);
    }
    #[tokio::test]
    async fn get_models_one_model_api_key_not_valid() {
        let client = Client::new("api_key");
        let response = client.get_models().await;
        assert!(response.is_err());
    }
    #[ignore = "This test requires a valid API key"]
    #[tokio::test]

    async fn get_models_one_model_not_valid_id() {
        dotenv::dotenv().ok();
        let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());
        let response = client.get_model_by_id("id").await;
        assert!(response.is_err());
    }
}
#[cfg(test)]
mod test_models_mock {

    use mockito::Server;

    use crate::Client;

    #[tokio::test]
    async fn get_models_mocked() {
        dotenv::dotenv().ok();
        let mut mock_server = Server::new_async().await;
        let mock_response = r#"{
            "object": "list",
            "data": [
                {
                    "id": "gpt-3.5-turbo",
                    "object": "model",
                    "created": 1623155845,
                    "owned_by": "openai"
                }
            ]
        }"#;
        mock_server
            .mock("GET", "/v1/models")
            .with_status(200)
            .match_header(
                "Authorization",
                format!("Bearer {}", std::env::var("OPEN_API_KEY").unwrap()).as_str(),
            )
            .with_body(mock_response)
            .create_async()
            .await;

        println!("Mock server: {}", mock_server.url());
        let client = Client::with_base_url(
            &std::env::var("OPEN_API_KEY").unwrap(),
            Some(&mock_server.url()),
        );
        let response = client.get_models().await.unwrap();
        assert!(response.object == "list");
        let first_model = response.data.first().unwrap();
        assert_eq!(first_model.id, "gpt-3.5-turbo");
    }
    #[tokio::test]
    async fn get_models_not_valid_api_key_mock() {
        let mut mock_server = Server::new_async().await;
        let mock_response = r#"{
            "error": {
                "param": "api_key",
                "message": "Invalid API key",
                "type": "invalid_request_error",
                "code": "invalid_api_key"
            }
        }"#;
        mock_server
            .mock("GET", "/v1/models")
            .match_header("Authorization", "Bearer api_key")
            .with_status(401)
            .with_body(mock_response)
            .create_async()
            .await;

        let client = Client::with_base_url("api_key", Some(&mock_server.url()));
        let response = client.get_models().await;
        assert!(response.is_err());
        if let Err(res) = response {
            match res {
                crate::structs::errors::ClientError::ApiError(e) => {
                    assert_eq!(e.error.error_type, "invalid_request_error");
                    assert_eq!(e.error.code, "invalid_api_key");
                }
                _ => panic!("Error type not expected"),
            }
        }
    }
    #[tokio::test]
    async fn get_models_one_model() {
        dotenv::dotenv().ok();
        let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());

        let response = client.get_models().await.unwrap();
        assert!(response.object == "list");
        let first_model = response.data.first().unwrap();
        let response_one_model = client.get_model_by_id(&first_model.id).await.unwrap();
        assert!(response_one_model.id == first_model.id);
    }
    #[tokio::test]
    async fn get_models_one_model_api_key_not_valid() {
        let client = Client::new("api_key");
        let response = client.get_models().await;
        assert!(response.is_err());
    }
    #[tokio::test]

    async fn get_models_one_model_not_valid_id() {
        dotenv::dotenv().ok();
        let client = Client::new(&std::env::var("OPEN_API_KEY").unwrap());
        let response = client.get_model_by_id("id").await;
        assert!(response.is_err());
    }
}
