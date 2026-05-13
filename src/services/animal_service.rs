use std::collections::HashMap;

use sqlx::Error;

use crate::{
    models::animals::{
        Animal, ParamsForAnimal, ParamsForAnimals
    }, 
    repositories::animal_repository::AnimalRepository, 
    schemas::CreateAnimalRequest
};


#[derive(Clone)]
pub struct AnimalService {
    pub repository: AnimalRepository
}

impl AnimalService {
    pub fn new(repository: AnimalRepository) -> Self {
        AnimalService{
            repository: repository
        }
    }

    pub async fn get_all(self, params: HashMap<String, String>) -> Result<Vec<Animal>, Error> {
        let params_struct = ParamsForAnimals {
            category: params.get("category").cloned(),
            health_symbol: params.get("health_symbol").cloned(),
            health_value: params.get("health_value")
                .and_then(|v| v.parse().ok()),
            satiety_symbol: params.get("satiety_symbol").cloned(),
            satiety_value: params.get("satiety_value")
                .and_then(|v| v.parse().ok()),
        };

        let result = self.repository.get_all_animals_by_params(params_struct).await?;

        Ok(result)
    }

    pub async fn get(self, params: HashMap<String, String>) -> Result<Animal, Error> {
        let params_struct = ParamsForAnimal {
            name: params.get("name").cloned(),
            id: params.get("id")
                .and_then(|v| v.parse().ok()),
        };

        let result = self.repository.get_animal_by_params(params_struct).await?;

        Ok(result)
    }

    pub async fn delete(self, id: i32) -> Result<(), Error> {
        self.repository.delete_animal_by_id(id).await?;
        Ok(())
    }

    pub async fn update(self, data: Animal) -> Result<(), Error> {
        self.repository.update_animal(data).await?;
        Ok(())
    }

    pub async fn create(self, data: CreateAnimalRequest) -> Result<i32, Error> {
        let id = self.repository.create_animal(data).await?;
        Ok(id)
    }
}