use axum::{Json, Router, extract::{Path, State}, routing::{get, post}, response::IntoResponse, http::StatusCode};
use tokio::net::TcpListener;
use std::{sync::Arc, time::Duration};
use sqlx::postgres::{PgPoolOptions, PgPool};
use dotenvy::dotenv;
use std::env;
use serde::{Serialize, Deserialize};
use sqlx::migrate::Migrator;
use std::path::Path as PathMigration;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Animal {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub health: i16,
    pub satiety: i16,
}

#[derive(Clone)]
struct DbState {
    pool: PgPool
}


impl IntoResponse for Animal {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}


async fn run_migrations_from_path(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migrator = Migrator::new(PathMigration::new("./migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}


async fn get_all_animals(State(state): State<Arc<DbState>>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let animals = sqlx::query_as::<_, Animal>("SELECT * FROM animals")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?;
    
    Ok((StatusCode::OK, Json(animals)))
}

async fn get_animal_by_id(
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

async fn get_animal_by_name(
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

async fn get_animals_by_category(
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

async fn delete_animal(
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

async fn update_animal(
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

async fn create_animal(
    State(state): State<Arc<DbState>>,
    Json(body): Json<Animal>
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

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    let _ = run_migrations_from_path(&pool)
        .await;

    let state = Arc::new(DbState { pool });

    let app = Router::new()
        .route("/animals", get(get_all_animals))
        .route("/animals/{category}", get(get_animals_by_category))
        .route("/animal", post(create_animal))
        .route("/animal/name/{name}", get(get_animal_by_name))
        .route("/animal/id/{id}", 
            get(get_animal_by_id)
            .delete(delete_animal)
            .put(update_animal)
        )
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}