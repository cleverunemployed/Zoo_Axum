use axum::{Router, routing::{get, post}};
use sqlx::PgPool;

use crate::{handlers::prelude::*, repositories::animal_repository::AnimalRepository, services::animal_service::AnimalService, state::AnimalState};



pub struct RouterAnimal {

}

impl RouterAnimal {
    pub fn new(pool: PgPool) -> Router {

        let state = AnimalState{
            service: AnimalService::new(
                AnimalRepository::new(
                    pool
                )
            )
        };

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

        app
    }
}