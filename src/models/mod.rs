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
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let models = response.json::<Model>().await?;
            Ok(models)
        } else {
            let error = response.json::<Error>().await.unwrap_or_else(|_| Error {
                error: ErrorData {
                    param: "Unknown".to_string(),
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
                    param: "Unknown".to_string(),
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
mod test_models {
    use crate::Client;

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
