use crate::services::{animal_service::AnimalService, user_service::UserService};


#[derive(Clone)]
pub struct AnimalState {
    pub service: AnimalService
}

#[derive(Clone)]
pub struct UserState {
    pub service: UserService
}