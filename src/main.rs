use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod db;
mod handlers;
mod models;
mod repositories;
mod routers;
mod schemas;
mod services;
mod state;

#[cfg(test)]
mod tests;

use config::Config;
use db::DBController;
use handlers::prelude::*;
use models::prelude::Animal;
use routers::prelude::*;
use schemas::CreateAnimalRequest;

use crate::schemas::{CreateUserRequest, UserResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Пути из animals
        get_all_animals,
        get_animals_by_category,
        get_animal_by_name,
        get_animal_by_id,
        create_animal,
        update_animal,
        delete_animal,
        // Пути из users
        get_all_users,
        get_user_by_credentials,
        delete_user,
        create_user,
    ),
    components(
        schemas(
            Animal, 
            CreateAnimalRequest,
            UserResponse, 
            CreateUserRequest
        )
    ),
    tags(
        (name = "animals", description = "Animal management endpoints"),
        (name = "users", description = "User management endpoints")
    ),

    info(
        title = "Combined API",
        description = "A REST API for managing zoo animals and users",
        version = "1.0.0",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
pub struct CombinedApiDoc;

#[tokio::main]
async fn main() {
    let env_config = Config::new();

    let database_url = env_config.database_url;

    let pool = Arc::new(DBController::new().get_pg_pool(database_url).await);

    let api_doc = CombinedApiDoc::openapi();

    let app = Router::new()
        .merge(RouterAnimal::new(Arc::clone(&pool)))
        .merge(RouterUser::new(Arc::clone(&pool)))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", env_config.port))
        .await
        .unwrap();

    println!(
        "Server listening on: http://{}",
        listener.local_addr().unwrap()
    );
    println!("Swagger UI available at: http://localhost:3000/swagger-ui");

    axum::serve(listener, app).await.unwrap();
}
