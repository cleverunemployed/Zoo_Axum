use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[schema(example = json!({
    "id": 1,
    "name": "Leo",
    "category": "lion",
    "health": 100,
    "satiety": 85
}))]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub health: i16,
    pub satiety: i16,
}

impl IntoResponse for Animal {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}