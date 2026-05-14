use std::collections::HashMap;

use crate::{models::users::{ParamsForUsers, User}, repositories::user_repository::UserRepository, schemas::{CreateUserRequest, UserResponse}};
use sqlx::Error;


#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService{
            repository: repository
        }
    }

    pub async fn get_all(self, params: HashMap<String, String>) -> Result<Vec<User>, Error> {
        let params_struct = ParamsForUsers {
            role: params.get("role").cloned(),
            is_deleted: params.get("health_value")
                .and_then(|v| v.parse().ok()),
        };

        let result = self.repository.get_all_users_by_params(params_struct).await?;

        Ok(result)
    }

    pub async fn get(self, params: HashMap<String, String>) -> Result<UserResponse, Error> {
        let params_struct = CreateUserRequest { 
            email: params.get("email").cloned().ok_or_else(|| Error::InvalidArgument("Not founded email!".to_string()))?, 
            password: params.get("password").cloned().ok_or_else(|| Error::InvalidArgument("Not founded password!".to_string()))?, 
        };

        let result = self.repository.get_user(params_struct).await?;

        Ok(result)
    }

    pub async fn delete(self, id: i32) -> Result<i32, Error> {
        let result = self.repository.delete_user(id).await?;

        Ok(result)
    }

    pub async fn create(self, params: HashMap<String, String>) -> Result<UserResponse, Error> {
        let params_struct = CreateUserRequest { 
            email: params.get("email").cloned().ok_or_else(|| Error::InvalidArgument("Not founded email!".to_string()))?, 
            password: params.get("password").cloned().ok_or_else(|| Error::InvalidArgument("Not founded password!".to_string()))?, 
        };

        let result = self.repository.create_user(params_struct).await?;

        Ok(result)
    }


}