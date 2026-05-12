use axum::{Json, Router, extract::{Path, State}, routing::{get, post}, response::IntoResponse, http::StatusCode};
use tokio::net::TcpListener;
use std::{sync::Arc, time::Duration};
use sqlx::postgres::{PgPoolOptions, PgPool};
use dotenvy::dotenv;
use std::env;
use serde::{Serialize, Deserialize};
use sqlx::migrate::Migrator;
use std::path::Path as PathMigration;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[schema(example = json!({
    "id": 1,
    "name": "Leo",
    "category": "lion",
    "health": 100,
    "satiety": 85
}))]
struct Animal {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub health: i16,
    pub satiety: i16,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CreateAnimalRequest {
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


#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_animals,
        get_animals_by_category,
        get_animal_by_name,
        get_animal_by_id,
        create_animal,
        update_animal,
        delete_animal
    ),
    components(
        schemas(Animal, CreateAnimalRequest)
    ),
    tags(
        (name = "animals", description = "Animal management endpoints")
    ),
    info(
        title = "Animals API",
        description = "A REST API for managing zoo animals with health and satiety tracking",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

async fn run_migrations_from_path(pool: &PgPool) -> Result<(), sqlx::Error> {
    let migrator = Migrator::new(PathMigration::new("./migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}


#[utoipa::path(
    get,
    path = "/animals",
    tag = "animals",
    responses(
        (status = 200, description = "List of all animals", body = [Animal]),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_all_animals(State(state): State<Arc<DbState>>) -> Result<impl IntoResponse, (StatusCode, String)> {
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
async fn create_animal(
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


    let api_doc = ApiDoc::openapi();

    let app = Router::new()
        // API endpoints
        .route("/animals", get(get_all_animals))
        .route("/animals/{category}", get(get_animals_by_category))
        .route("/animal", post(create_animal))
        .route("/animal/name/{name}", get(get_animal_by_name))
        .route("/animal/id/{id}", 
            get(get_animal_by_id)
            .delete(delete_animal)
            .put(update_animal)
        )
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_doc)
        )
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server listening on: http://{}", listener.local_addr().unwrap());
    println!("Swagger UI available at: http://localhost:3000/swagger-ui");

    axum::serve(listener, app).await.unwrap();
}