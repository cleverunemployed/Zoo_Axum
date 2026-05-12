use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAnimalRequest {
    pub name: String,
    pub category: String,
    pub health: i16,
    pub satiety: i16,
}

