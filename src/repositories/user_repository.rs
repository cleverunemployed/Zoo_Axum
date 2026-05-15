use std::sync::Arc;

use crate::{
    models::users::{ParamsForUsers, User},
    schemas::{CreateUserRequest, DeleteUserRequest, UserResponse},
};
use sqlx::{PgPool, QueryBuilder, error::Error};

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserRepository { pool: pool }
    }

    pub async fn get_all_users_by_params(self, params: ParamsForUsers) -> Result<Vec<User>, Error> {
        let mut query_builder = QueryBuilder::<sqlx::Postgres>::new("SELECT * FROM users");

        let mut has_conditions = false;
        let mut seperated = query_builder.separated("");

        if let Some(role) = params.role {
            if !has_conditions {
                seperated.push(" WHERE ");
                has_conditions = true;
            }
            seperated.push("role = ").push_bind(role);
        }

        if let Some(is_deleted) = params.is_deleted {
            if !has_conditions {
                seperated.push(" WHERE ");
            } else {
                seperated.push(" AND ");
            }
            seperated.push("is_deleted = ").push_bind(is_deleted);
        }

        let query = query_builder.build_query_as::<User>();

        let users = query.fetch_all(&*self.pool).await?;

        Ok(users)
    }

    pub async fn get_user(self, data: &CreateUserRequest) -> Result<UserResponse, Error> {
        let user = sqlx::query_as!(
            UserResponse,
            "SELECT id, email, role FROM users WHERE email = $1 AND password = $2",
            data.email,
            data.password
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(self, data: CreateUserRequest) -> Result<UserResponse, Error> {

        println!("{:#?}", data);
        let response = sqlx::query_as!(
            UserResponse,
            "INSERT INTO users (email, password, role, is_deleted) 
            VALUES ($1, $2, $3, $4) 
            RETURNING id, email, role",
            data.email, data.password, "player", false
        )
        .fetch_optional(&*self.pool)
        .await?;

        println!("{:#?}", response);

        match response {
            Some(user) => Ok(user),
            None => Err(Error::InvalidArgument("Failed to create user".to_string())),
        }
    }

    pub async fn delete_user(self, data: DeleteUserRequest) -> Result<i32, Error> {
        let result = sqlx::query!(
            "UPDATE users SET is_deleted = true WHERE id = $1 and password = $2",
            data.id,
            data.password
        )
        .execute(&*self.pool)
        .await?;

        if result.rows_affected() == 0 {
            println!("User not found");
        }

        Ok(data.id)
    }

    // pub async fn update_user(self, data: UpdateUserRequest) -> Result<UserResponse, Error> {
    //     let mut query_builder = QueryBuilder::<sqlx::Postgres>::new("UPDATE users SET");

    //     let mut needs_comma = false;

    //     if let Some(password) = data.new_password {
    //         if needs_comma {
    //             query_builder.push(",");
    //         }
    //         query_builder.push(" password = ").push_bind(password);
    //         needs_comma = true;
    //     }

    //     if let Some(role) = data.role {
    //         if needs_comma {
    //             query_builder.push(",");
    //         }
    //         query_builder.push(" role = ").push_bind(role);
    //         needs_comma = true;
    //     }

    //     if !needs_comma {
    //         return Err(Error::ColumnNotFound(
    //             "Для обновления пользователя нужны данные!".to_string(),
    //         ));
    //     }

    //     query_builder.push(" WHERE id = ").push_bind(data.id);
    //     query_builder.push(" RETURNING id, role, email");

    //     let response = query_builder
    //         .build_query_as::<UserResponse>()
    //         .fetch_one(&*self.pool)
    //         .await?;

    //     Ok(response)
    // }
}
