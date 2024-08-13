use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub object: String,
    pub data: Vec<ModelData>,
}
#[derive(Debug, Serialize, Deserialize)]

pub struct ModelData {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}
