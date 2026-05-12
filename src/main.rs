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

fn setup_app(state: Arc<DbState>) -> Router {
    let api_doc = ApiDoc::openapi();

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
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_doc)
        )
        .with_state(state);

    app
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

    let app = setup_app(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server listening on: http://{}", listener.local_addr().unwrap());
    println!("Swagger UI available at: http://localhost:3000/swagger-ui");

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;
    use axum::http::StatusCode;
    use super::*;
    use serde_json::json;

    async fn setup_test_db() -> Arc<DbState> {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/animals_test".to_string());
        
        // Create a test database connection
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .connect(&database_url)
            .await
            .expect("Failed to create test database connection pool");

        // Run migrations for test database
        let _ = run_migrations_from_path(&pool).await;

        // Clean up any existing data
        let _ = sqlx::query("DELETE FROM animals")
            .execute(&pool)
            .await;

        Arc::new(DbState { pool })
    }

    async fn create_test_animal(server: &TestServer, name: &str, category: &str, health: i16, satiety: i16) -> Animal {
        let response = server
            .post("/animal")
            .json(&json!({
                "name": name,
                "category": category,
                "health": health,
                "satiety": satiety
            }))
            .await;
        
        response.assert_status(StatusCode::CREATED);
        response.json::<Animal>()
    }

    #[tokio::test]
    async fn test_create_animal() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let new_animal = json!({
            "name": "Simba",
            "category": "lion",
            "health": 100,
            "satiety": 85
        });

        let response = server.post("/animal").json(&new_animal).await;
        response.assert_status(StatusCode::CREATED);
        
        let animal: Animal = response.json();
        assert_eq!(animal.name, "Simba");
        assert_eq!(animal.category, "lion");
        assert_eq!(animal.health, 100);
        assert_eq!(animal.satiety, 85);
        assert!(animal.id > 0);
    }

    #[tokio::test]
    async fn test_create_animal_invalid_data() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        // Missing required fields
        let invalid_animal = json!({
            "name": "Simba"
        });

        let response = server.post("/animal").json(&invalid_animal).await;
        // Should return 400 Bad Request or 422 Unprocessable Entity
        assert!(response.status_code() == StatusCode::BAD_REQUEST || 
                response.status_code() == StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_get_all_animals() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        // Create test animals
        create_test_animal(&server, "Leo", "lion", 100, 85).await;
        create_test_animal(&server, "Tigra", "tiger", 95, 90).await;
        create_test_animal(&server, "Baloo", "bear", 80, 75).await;

        let response = server.get("/animals").await;
        response.assert_status(StatusCode::OK);
        
        let animals: Vec<Animal> = response.json();
        assert!(animals.len() >= 3);
        
        let names: Vec<String> = animals.iter().map(|a| a.name.clone()).collect();

        println!("{:#?}", names);

        assert!(names.contains(&"Leo".to_string()));
        assert!(names.contains(&"Tigra".to_string()));
        assert!(names.contains(&"Baloo".to_string()));
    }

    #[tokio::test]
    async fn test_get_animal_by_id() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let created = create_test_animal(&server, "Simba", "lion", 100, 85).await;

        let response = server.get(&format!("/animal/id/{}", created.id)).await;
        response.assert_status(StatusCode::OK);
        
        let animal: Animal = response.json();
        assert_eq!(animal.id, created.id);
        assert_eq!(animal.name, "Simba");
    }

    #[tokio::test]
    async fn test_get_animal_by_id_not_found() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let response = server.get("/animal/id/99999").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_animal_by_name() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        create_test_animal(&server, "UniqueName", "lion", 100, 85).await;

        let response = server.get("/animal/name/UniqueName").await;
        response.assert_status(StatusCode::OK);
        
        let animal: Animal = response.json();
        assert_eq!(animal.name, "UniqueName");
        assert_eq!(animal.category, "lion");
    }

    #[tokio::test]
    async fn test_get_animal_by_name_not_found() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let response = server.get("/animal/name/NonExistentAnimal").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_animals_by_category() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        // Create multiple animals in same category
        create_test_animal(&server, "Leo1", "lion", 100, 85).await;
        create_test_animal(&server, "Leo2", "lion", 95, 80).await;
        create_test_animal(&server, "Tigra", "tiger", 90, 85).await;

        let response = server.get("/animals/lion").await;
        response.assert_status(StatusCode::OK);
        
        let animals: Vec<Animal> = response.json();
        assert_eq!(animals.len(), 2);
        for animal in animals {
            assert_eq!(animal.category, "lion");
        }
    }

    #[tokio::test]
    async fn test_get_animals_by_category_empty() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let response = server.get("/animals/nonexistent").await;
        response.assert_status(StatusCode::OK);
        
        let animals: Vec<Animal> = response.json();
        assert!(animals.is_empty());
    }

    #[tokio::test]
    async fn test_update_animal() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let created = create_test_animal(&server, "Simba", "lion", 100, 85).await;

        let updated_data = json!({
            "id": created.id,
            "name": "Simba Updated",
            "category": "tiger",
            "health": 75,
            "satiety": 60
        });

        let response = server
            .put(&format!("/animal/id/{}", created.id))
            .json(&updated_data)
            .await;
        
        response.assert_status(StatusCode::OK);
        
        let updated: Animal = response.json();
        assert_eq!(updated.name, "Simba Updated");
        assert_eq!(updated.category, "tiger");
        assert_eq!(updated.health, 75);
        assert_eq!(updated.satiety, 60);
        assert_eq!(updated.id, created.id);

        // Verify with GET request
        let get_response = server.get(&format!("/animal/id/{}", created.id)).await;
        let verified: Animal = get_response.json();
        assert_eq!(verified.name, "Simba Updated");
    }

    #[tokio::test]
    async fn test_update_animal_not_found() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let updated_data = json!({
            "id": 99999,
            "name": "Ghost",
            "category": "ghost",
            "health": 0,
            "satiety": 0
        });

        let response = server.put("/animal/id/99999").json(&updated_data).await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_animal() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let created = create_test_animal(&server, "ToDelete", "lion", 100, 85).await;

        // Delete the animal
        let delete_response = server.delete(&format!("/animal/id/{}", created.id)).await;
        delete_response.assert_status(StatusCode::NO_CONTENT);

        // Verify it's gone
        let get_response = server.get(&format!("/animal/id/{}", created.id)).await;
        get_response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_animal_not_found() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        let response = server.delete("/animal/id/99999").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_multiple_operations_sequence() {
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = TestServer::new(app);

        // 1. Create animal
        let animal = create_test_animal(&server, "Sequence", "zebra", 100, 100).await;
        
        // 2. Get by ID
        let get_response = server.get(&format!("/animal/id/{}", animal.id)).await;
        get_response.assert_status(StatusCode::OK);
        
        // 3. Update animal
        let update_data = json!({
            "id": animal.id,
            "name": "Sequence Updated",
            "category": "zebra",
            "health": 50,
            "satiety": 50
        });
        let update_response = server.put(&format!("/animal/id/{}", animal.id)).json(&update_data).await;
        update_response.assert_status(StatusCode::OK);
        
        // 4. Verify update
        let verify_response = server.get(&format!("/animal/id/{}", animal.id)).await;
        let verified: Animal = verify_response.json();
        assert_eq!(verified.health, 50);
        
        // 5. Delete animal
        let delete_response = server.delete(&format!("/animal/id/{}", animal.id)).await;
        delete_response.assert_status(StatusCode::NO_CONTENT);
        
        // 6. Verify deletion
        let final_response = server.get(&format!("/animal/id/{}", animal.id)).await;
        final_response.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        use std::sync::Arc;
        use tokio::task;
        
        let state = setup_test_db().await;
        let app = setup_app(state);
        let server = Arc::new(TestServer::new(app));

        let mut handles = vec![];

        // Create 10 animals concurrently
        for i in 0..10 {
            let server_ref = Arc::clone(&server);
            let handle = task::spawn(async move {
                let animal = json!({
                    "name": format!("Concurrent_{}", i),
                    "category": "test",
                    "health": 100,
                    "satiety": 100
                });
                let response = server_ref.post("/animal").json(&animal).await;
                assert_eq!(response.status_code(), StatusCode::CREATED);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all were created
        let response = server.get("/animals").await;
        let animals: Vec<Animal> = response.json();
        let test_animals: Vec<&Animal> = animals.iter().filter(|a| a.category == "test").collect();
        assert_eq!(test_animals.len(), 10);
    }
}