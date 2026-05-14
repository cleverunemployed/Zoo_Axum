use axum::{Router};
use tokio::net::TcpListener;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod config;
mod schemas;
mod models;
mod handlers;
mod routers;
mod services;
mod repositories;
mod state;


#[cfg(test)]
mod tests;

use config::Config;
use db::{DBController};
use schemas::CreateAnimalRequest;
use models::prelude::Animal;
use handlers::prelude::*;
use routers::prelude::*;



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



#[tokio::main]
async fn main() {

    let env_config = Config::new();

    let database_url = env_config.database_url;

    let pool = DBController::new()
        .get_pg_pool(database_url)
        .await;

    let api_doc = ApiDoc::openapi();

    let app = Router::new()
        .merge(RouterAnimal::new(pool))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", api_doc)
        );

    let listener = TcpListener::bind(format!("0.0.0.0:{}", env_config.port)).await.unwrap();

    println!("Server listening on: http://{}", listener.local_addr().unwrap());
    println!("Swagger UI available at: http://localhost:3000/swagger-ui");

    axum::serve(listener, app).await.unwrap();
}

