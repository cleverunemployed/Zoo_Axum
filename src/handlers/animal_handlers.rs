use std::collections::HashMap;

use crate::{
    models::animals::Animal,
    schemas::{CreateAnimalRequest, GetAnimalsRequest},
    state::AnimalState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
// use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/animals",
    tag = "animals",
    request_body = GetAnimalsRequest,
    responses(
        (status = 200, description = "List of all animals", body = [Animal]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_animals(
    State(state): State<AnimalState>,
    Json(data): Json<GetAnimalsRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut params: HashMap<String, String> = HashMap::new();

    params.insert("id".to_string(), data.id.to_string());

    let animals = state.service.get_all(params).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Животные не найдены".to_string()),
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::OK, Json(animals)))
}

#[utoipa::path(
    get,
    path = "/animal/id/{id}",
    tag = "animals",
    params(
        ("id" = i32, Path, description = "Animal database ID")
    ),
    responses(
        (status = 200, description = "Animal found", body = Animal),
        (status = 404, description = "Animal not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_animal_by_id(
    State(state): State<AnimalState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut params: HashMap<String, String> = HashMap::new();

    params.insert("id".to_string(), id.to_string());

    let animal = state.service.get(params).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Животное не найдено".to_string()),
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::OK, Json(animal)))
}

#[utoipa::path(
    get,
    path = "/animal/name/{name}",
    tag = "animals",
    params(
        ("name" = String, Path, description = "Animal name")
    ),
    responses(
        (status = 200, description = "Animal found", body = Animal),
        (status = 404, description = "Animal not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_animal_by_name(
    State(state): State<AnimalState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut params: HashMap<String, String> = HashMap::new();

    params.insert("name".to_string(), name.to_string());

    let animal = state.service.get(params).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Животное не найдено".to_string()),
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::OK, Json(animal)))
}

#[utoipa::path(
    get,
    path = "/animals/{category}",
    tag = "animals",
    params(
        ("category" = String, Path, description = "Animal category (e.g., lion, tiger, bear)")
    ),
    responses(
        (status = 200, description = "List of animals in category", body = [Animal]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_animals_by_category(
    State(state): State<AnimalState>,
    Path(category): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut params: HashMap<String, String> = HashMap::new();

    params.insert("category".to_string(), category);

    let animals = state.service.get_all(params).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Животные не найдены".to_string()),
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::OK, Json(animals)))
}

#[utoipa::path(
    delete,
    path = "/animal/id/{id}",
    tag = "animals",
    params(
        ("id" = i32, Path, description = "Animal database ID")
    ),
    responses(
        (status = 204, description = "Animal successfully deleted"),
        (status = 404, description = "Animal not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_animal(
    State(state): State<AnimalState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.service.delete(id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Животное не найдено".to_string()),
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::NO_CONTENT, "Animal deleted"))
}

#[utoipa::path(
    put,
    path = "/animal/id/{id}",
    tag = "animals",
    params(
        ("id" = i32, Path, description = "Animal database ID")
    ),
    request_body = Animal,
    responses(
        (status = 200, description = "Animal successfully updated", body = Animal),
        (status = 404, description = "Animal not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_animal(
    State(state): State<AnimalState>,
    Path(id): Path<i32>,
    Json(body): Json<Animal>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut data = body;
    data.id = id;

    state.service.update(data).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Животное не найдено".to_string()),
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::NO_CONTENT, "Animal updated"))
}

#[utoipa::path(
    post,
    path = "/animal",
    tag = "animals",
    request_body = CreateAnimalRequest,
    responses(
        (status = 201, description = "Animal successfully created", body = Animal),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_animal(
    State(state): State<AnimalState>,
    Json(body): Json<CreateAnimalRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = state.service.create(body).await.map_err(|e| match e {
        sqlx::Error::Database(db_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка БД: {}", db_err),
        ),
        other => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ошибка: {}", other),
        ),
    })?;

    Ok((StatusCode::CREATED, Json(id)))
}

// #[derive(OpenApi)]
// #[openapi(
//     paths(
//         get_all_animals,
//         get_animals_by_category,
//         get_animal_by_name,
//         get_animal_by_id,
//         create_animal,
//         update_animal,
//         delete_animal
//     ),
//     components(
//         schemas(Animal, CreateAnimalRequest)
//     ),
//     tags(
//         (name = "animals", description = "Animal management endpoints")
//     ),
//     info(
//         title = "Animals API",
//         description = "A REST API for managing zoo animals with health and satiety tracking",
//         version = "1.0.0",
//         contact(
//             name = "API Support",
//             email = "support@example.com"
//         )
//     )
// )]
// pub struct ApiDoc;
