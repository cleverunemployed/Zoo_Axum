use axum::{Json, extract::{Path, State}, response::IntoResponse, http::StatusCode};
use std::{sync::Arc};

use crate::{db::DbState, models::animals::Animal, schemas::CreateAnimalRequest};



#[utoipa::path(
    get,
    path = "/animals",
    tag = "animals",
    responses(
        (status = 200, description = "List of all animals", body = [Animal]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_animals(State(state): State<Arc<DbState>>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let animals = sqlx::query_as::<_, Animal>("SELECT * FROM animals")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
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
    State(state): State<Arc<DbState>>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let animal = sqlx::query_as::<_, Animal>("SELECT * FROM animals WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    match animal {
        Some(animal) => Ok((StatusCode::OK, Json(animal))),
        None => Err((StatusCode::NOT_FOUND, format!("Animal with id {} not found", id))),
    }
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
    State(state): State<Arc<DbState>>,
    Path(name): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let animal = sqlx::query_as::<_, Animal>("SELECT * FROM animals WHERE name = $1")
        .bind(&name)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    match animal {
        Some(animal) => Ok((StatusCode::OK, Json(animal))),
        None => Err((StatusCode::NOT_FOUND, format!("Animal with name '{}' not found", name))),
    }
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
    State(state): State<Arc<DbState>>,
    Path(category): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let animals = sqlx::query_as::<_, Animal>("SELECT * FROM animals WHERE category = $1")
        .bind(&category)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
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
    State(state): State<Arc<DbState>>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM animals WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, format!("Animal with id {} not found", id)))
    } else {
        Ok((StatusCode::NO_CONTENT, "Animal deleted"))
    }
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
    State(state): State<Arc<DbState>>,
    Path(id): Path<i32>,
    Json(body): Json<Animal>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query(
        "UPDATE animals SET name = $1, category = $2, health = $3, satiety = $4 WHERE id = $5"
    )
        .bind(&body.name)
        .bind(&body.category)
        .bind(body.health)
        .bind(body.satiety)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, format!("Animal with id {} not found", id)))
    } else {
        let updated_animal = sqlx::query_as::<_, Animal>("SELECT * FROM animals WHERE id = $1")
            .bind(id)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
        
        Ok((StatusCode::OK, Json(updated_animal)))
    }
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
    State(state): State<Arc<DbState>>,
    Json(body): Json<CreateAnimalRequest>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let animal = sqlx::query_as::<_, Animal>(
        "INSERT INTO animals (name, category, health, satiety) VALUES ($1, $2, $3, $4) RETURNING *"
    )
        .bind(&body.name)
        .bind(&body.category)
        .bind(body.health)
        .bind(body.satiety)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    Ok((StatusCode::CREATED, Json(animal)))
}
