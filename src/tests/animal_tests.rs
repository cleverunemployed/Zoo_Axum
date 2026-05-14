use crate::{db::DBController, models::animals::Animal, routers::animal_route::RouterAnimal};
use axum_test::TestServer;
use axum::{Router, http::StatusCode};
use serde_json::json;
use dotenvy::dotenv;
use std::env;
use sqlx::{self, PgPool};


async fn setup_test_db() -> PgPool {
    dotenv().ok();
    
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/animals_test".to_string());

    let pool = DBController::new().get_pg_pool(database_url).await;

    // Clean up any existing data
    let _ = sqlx::query("DELETE FROM animals")
        .execute(&pool)
        .await;

    pool
}

fn setup_app(pool: PgPool) -> Router {
    Router::new()
        .merge(RouterAnimal::new(pool))
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

    let id: i32 = response
        .text()
        .parse()
        .unwrap();

    println!("Returned ID: {}", id);
    assert!(id > 0, "ID должен быть положительным числом");

}
