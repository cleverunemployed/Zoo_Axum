use axum::{Json, extract::{Path, Query, State}, response::IntoResponse, http::StatusCode};
use std::collections::HashMap;
use crate::{schemas::{CreateUserRequest, UserResponse}, state::UserState};

use utoipa::OpenApi;



#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_users,
        get_user_by_credentials,
        delete_user,
        create_user
    ),
    tags(
        (name = "users", description = "User management endpoints")
    ),
    components(
        schemas(UserResponse, CreateUserRequest)
    )
)]
pub struct ApiDoc;


#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    params(
        ("filter" = Option<String>, Query, description = "Optional filter parameters"),
        ("sort" = Option<String>, Query, description = "Sorting criteria")
    ),
    responses(
        (status = 200, description = "List of users retrieved successfully", body = Vec<UserResponse>),
        (status = 500, description = "Internal server error", body = String, example = json!("Ошибка БД: ..."))
    )
)]

pub async fn get_all_users(
    State(state): State<UserState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = state.service
        .clone()
        .get_all(params)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка БД: {}", db_err))
            }
            other => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка: {}", other))
            }
        })?;

    Ok((StatusCode::OK, Json(users)))
}

#[utoipa::path(
    get,
    path = "/users/credentials",
    tag = "users",
    params(
        ("email" = String, Query, description = "User email address"),
        ("password" = String, Query, description = "User password")
    ),
    responses(
        (status = 200, description = "User found successfully", body = UserResponse),
        (status = 400, description = "Email and password are required", body = String, example = json!("Email и password обязательны")),
        (status = 404, description = "User not found", body = String, example = json!("Пользователь не найден")),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn get_user_by_credentials(
    State(state): State<UserState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !params.contains_key("email") || !params.contains_key("password") {
        return Err((StatusCode::BAD_REQUEST, "Email и password обязательны".to_string()));
    }

    let user = state.service
        .clone()
        .get(params)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                (StatusCode::NOT_FOUND, "Пользователь не найден".to_string())
            }
            sqlx::Error::Database(db_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка БД: {}", db_err))
            }
            other => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка: {}", other))
            }
        })?;

    Ok((StatusCode::OK, Json(user)))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User database ID", minimum = 1)
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn delete_user(
    State(state): State<UserState>,
    Path(user_id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let deleted_id = state.service
        .clone()
        .delete(user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка БД: {}", db_err))
            }
            other => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка: {}", other))
            }
        })?;

    Ok((StatusCode::OK, Json(serde_json::json!({
        "message": "Пользователь успешно удален",
        "user_id": deleted_id
    }))))
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Email or password missing", body = String, example = json!("Email обязателен")),
        (status = 409, description = "User with this email already exists", body = String, example = json!("Пользователь с таким email уже существует")),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn create_user(
    State(state): State<UserState>,
    Json(body): Json<serde_json::Value>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let params = HashMap::from([
        ("email".to_string(), body.get("email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| (StatusCode::BAD_REQUEST, "Email обязателен".to_string()))?
            .to_string()),
        ("password".to_string(), body.get("password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| (StatusCode::BAD_REQUEST, "Password обязателен".to_string()))?
            .to_string()),
    ]);

    let user = state.service
        .clone()
        .create(params)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                if db_err.constraint().is_some() {
                    (StatusCode::CONFLICT, "Пользователь с таким email уже существует".to_string())
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка БД: {}", db_err))
                }
            }
            other => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка: {}", other))
            }
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}
