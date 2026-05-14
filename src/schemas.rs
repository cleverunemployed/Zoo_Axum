use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use utoipa::{ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAnimalRequest {
    pub name: String,
    pub category: String,
    pub health: i16,
    pub satiety: i16,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub old_password: String,
    pub new_password: Option<String>,
    pub role: Option<String>
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteUserRequest {
    pub id: i32,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub role: String
}



