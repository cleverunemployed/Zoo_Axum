
use crate::{models::animals::{Animal, ParamsForAnimal, ParamsForAnimals}, schemas::CreateAnimalRequest};
use sqlx::{PgPool, QueryBuilder, error::Error};

#[derive(Clone)]
pub struct AnimalRepository {
    pub pool: PgPool
}

impl AnimalRepository {
    pub fn new(pool: PgPool) -> Self {
        AnimalRepository{
            pool: pool
        }
    }

    pub async fn get_all_animals_by_params(&self, params: ParamsForAnimals) -> Result<Vec<Animal>, Error> {
        let mut query_builder = QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM animals");
        
        let mut has_conditions = false;
        let mut seperated = query_builder.separated("");

        if let Some(category) = params.category {
            if !has_conditions {
                seperated.push(" WHERE ");
                has_conditions = true;
            }
            seperated.push("category = ").push_bind(category);
        }
        
        if let Some(health_symbol) = &params.health_symbol {
            if let Some(health_value) = params.health_value {
                if !has_conditions {
                    seperated.push(" WHERE ");
                    has_conditions = true;
                } else {
                    seperated.push(" AND ");
                }
                
                seperated.push(format!("health {} ", health_symbol)).push_bind(health_value);
            }
        }
        
        if let Some(satiety_symbol) = &params.satiety_symbol {
            if let Some(satiety_value) = params.satiety_value {
                if !has_conditions {
                    seperated.push(" WHERE ");
                } else {
                    seperated.push(" AND ");
                }
                seperated.push(format!("satiety {} ", satiety_symbol)).push_bind(satiety_value);
            }
        }
        
        let query = query_builder.build_query_as::<Animal>();

        let animals = query.fetch_all(&self.pool).await?;
        
        Ok(animals)
    }

    pub async fn get_animal_by_params(self, params: ParamsForAnimal) -> Result<Animal, Error> {

        let mut conditions = Vec::new();
        let mut query_builder = QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM animals");

        if let Some(id) = params.id {
            query_builder.push(" WHERE id = ");
            query_builder.push_bind(id);
            conditions.push(true);
        }
        
        if let Some(name) = params.name {
            if conditions.is_empty() {
                query_builder.push(" WHERE ");
            } else {
                query_builder.push(" AND ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(name);
            conditions.push(true);
        }
        
        if conditions.is_empty() {
            return Err(Error::RowNotFound);
        }
        
        let query = query_builder.build_query_as::<Animal>();
        let animal = query.fetch_one(&self.pool).await?;
        
        Ok(animal)
    }

    pub async fn delete_animal_by_id(self, id: i32) -> Result<(), Error> {
        sqlx::query!("DELETE FROM animals WHERE id = $1", id)
            .execute(&self.pool)  
            .await?;
    
        Ok(())
    }

    pub async fn create_animal(self, data: CreateAnimalRequest) -> Result<i32, Error> {
        let id = sqlx::query!(
                "INSERT INTO animals (name, category, health, satiety) VALUES ($1, $2, $3, $4) RETURNING id",
                data.name, data.category, data.health, data.satiety
            )
            .fetch_one(&self.pool)
            .await?
            .id;

        Ok(id)
    }

    pub async fn update_animal(self, data: Animal) -> Result<(), Error> {
        let result = sqlx::query!("UPDATE animals SET name = $1, category = $2, health = $3, satiety = $4 WHERE id = $5", data.name, data.category, data.health, data.satiety, data.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            println!("Animal not found");
        }
        Ok(())
    }
}