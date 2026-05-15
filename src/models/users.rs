use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// pub enum UserRole {
//     Player,
//     VipPlayer,
//     Admin,
// }

// impl UserRole {
//     pub fn name(&self) -> &str {
//         match self {
//             UserRole::Player => "player",
//             UserRole::VipPlayer => "vip",
//             UserRole::Admin => "admin",
//         }
//     }

//     pub fn value(&self) -> i32 {
//         match self {
//             UserRole::Player => 5,
//             UserRole::VipPlayer => 10,
//             UserRole::Admin => 15,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[schema(example = json!({
    "id": 1,
    "email": "zoo@zoo.com",
    "password": "******",
    "role": "player"
}))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role: String,
    pub is_deleted: bool,
}

impl IntoResponse for User {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
// #[schema(example = json!({
//     "id": 1,
//     "id_user": 1,
//     "id_animal": 1
// }))]
// pub struct UserAnimals {
//     pub id: i32,
//     pub id_user: i32,
//     pub id_animal: i32,
// }

// impl IntoResponse for UserAnimals {
//     fn into_response(self) -> axum::response::Response {
//         (StatusCode::OK, Json(self)).into_response()
//     }
// }

#[derive(Debug)]
pub struct ParamsForUsers {
    pub role: Option<String>,
    pub is_deleted: Option<bool>,
}
