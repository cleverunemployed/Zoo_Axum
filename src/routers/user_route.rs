use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get},
};
use sqlx::PgPool;

use crate::{
    handlers::prelude::*,
    repositories::user_repository::UserRepository,
    services::user_service::UserService,
    state::UserState,
};

pub struct RouterUser {}

impl RouterUser {
    pub fn new(pool: Arc<PgPool>) -> Router {
        let state = UserState {
            service: UserService::new(UserRepository::new(pool)),
        };

        let app = Router::new()
            .route("/users", get(get_all_users).post(create_user))
            .route("/users/credentials", get(get_user_by_credentials))
            .route("/users/{id}", delete(delete_user))
            .with_state(state);

        app
    }
}
