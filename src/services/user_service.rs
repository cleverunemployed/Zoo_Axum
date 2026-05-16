use std::collections::HashMap;

use crate::{
    hash_password::hash_password, models::users::{ParamsForUsers, User}, repositories::user_repository::UserRepository, schemas::{CreateUserRequest, DeleteUserRequest, UserResponse}
};
use sqlx::Error;

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService {
            repository: repository,
        }
    }

    pub async fn get_all(self, params: HashMap<String, String>) -> Result<Vec<User>, Error> {
        let params_struct = ParamsForUsers {
            role: params.get("role").cloned(),
            is_deleted: params.get("is_deleted").and_then(|v| v.parse().ok()),
        };

        let result = self
            .repository
            .get_all_users_by_params(params_struct)
            .await?;

        Ok(result)
    }

    pub async fn get(self, params: HashMap<String, String>) -> Result<UserResponse, Error> {
        let mut params_struct = CreateUserRequest {
            email: params
                .get("email")
                .cloned()
                .ok_or_else(|| Error::InvalidArgument("Not founded email!".to_string()))?,
            password: params
                .get("password")
                .cloned()
                .ok_or_else(|| Error::InvalidArgument("Not founded password!".to_string()))?,
        };

        params_struct.password = hash_password(params_struct.password);

        let result = self.repository.get_user(&params_struct).await?;

        Ok(result)
    }

    pub async fn delete(self, data: DeleteUserRequest) -> Result<i32, Error> {

        let mut data = data;
        data.password = hash_password(data.password);

        let result = self.repository.delete_user(data).await?;

        Ok(result)
    }

    pub async fn create(self, params: HashMap<String, String>) -> Result<UserResponse, Error> {
        let mut params_struct = CreateUserRequest {
            email: params
                .get("email")
                .cloned()
                .ok_or_else(|| Error::InvalidArgument("Not founded email!".to_string()))?,
            password: params
                .get("password")
                .cloned()
                .ok_or_else(|| Error::InvalidArgument("Not founded password!".to_string()))?,
        };

        params_struct.password = hash_password(params_struct.password);

        // let existing_user = self.repository.clone().get_user(&params_struct).await?;

        // println!("User {:#?}", existing_user);

        // if !existing_user.email.is_empty() {
        //     return Err(Error::InvalidArgument("User already exists".to_string()));
        // }

        let result = self.repository.create_user(params_struct).await?;

        Ok(result)
    }
}
