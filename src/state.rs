use crate::services::animal_service::AnimalService;


#[derive(Clone)]
pub struct AnimalState {
    pub service: AnimalService
}