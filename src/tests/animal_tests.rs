use crate::{db::{DBController, DbState}, models::animals::Animal, routers::animal_route::RouterAnimal};
use axum_test::TestServer;
use axum::{Router, http::StatusCode};
use serde_json::json;
use std::sync::Arc;
use dotenvy::dotenv;
use std::env;
use sqlx;


async fn setup_test_db() -> Arc<DbState> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/animals_test".to_string());
    
    let pool = DBController::new().get_pg_pool(database_url).await;

    // Clean up any existing data
    let _ = sqlx::query("DELETE FROM animals")
        .execute(&pool.pool)
        .await;

    pool
}

fn setup_app(state: Arc<DbState>) -> Router {
    Router::new()
        .merge(RouterAnimal::new(state))
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

    let invalid_animal = json!({
        "name": "Simba"
    });

    let response = server.post("/animal").json(&invalid_animal).await;
    assert!(response.status_code() == StatusCode::BAD_REQUEST || 
            response.status_code() == StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_get_all_animals() {
    let state = setup_test_db().await;
    let app = setup_app(state);
    let server = TestServer::new(app);

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

    let delete_response = server.delete(&format!("/animal/id/{}", created.id)).await;
    delete_response.assert_status(StatusCode::NO_CONTENT);

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

    let animal = create_test_animal(&server, "Sequence", "zebra", 100, 100).await;
    
    let get_response = server.get(&format!("/animal/id/{}", animal.id)).await;
    get_response.assert_status(StatusCode::OK);
    
    let update_data = json!({
        "id": animal.id,
        "name": "Sequence Updated",
        "category": "zebra",
        "health": 50,
        "satiety": 50
    });
    let update_response = server.put(&format!("/animal/id/{}", animal.id)).json(&update_data).await;
    update_response.assert_status(StatusCode::OK);
    
    let verify_response = server.get(&format!("/animal/id/{}", animal.id)).await;
    let verified: Animal = verify_response.json();
    assert_eq!(verified.health, 50);
    
    let delete_response = server.delete(&format!("/animal/id/{}", animal.id)).await;
    delete_response.assert_status(StatusCode::NO_CONTENT);
    
    let final_response = server.get(&format!("/animal/id/{}", animal.id)).await;
    final_response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_concurrent_requests() {
    use tokio::task;
    
    let state = setup_test_db().await;
    let app = setup_app(state);
    let server = Arc::new(TestServer::new(app));

    let mut handles = vec![];

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

    let response = server.get("/animals").await;
    let animals: Vec<Animal> = response.json();
    let test_animals: Vec<&Animal> = animals.iter().filter(|a| a.category == "test").collect();
    assert_eq!(test_animals.len(), 10);
}