use std::sync::Arc;
use axum::{Router, routing::{get, post}};
use crate::db::DbState;

use crate::handlers::prelude::*;


pub struct RouterAnimal {

}

impl RouterAnimal {
    pub fn new(state: Arc<DbState>) -> Router {
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