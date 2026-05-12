
use std::{sync::Arc, time::Duration};
use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::migrate::Migrator;
use std::path::Path as PathMigration;


#[derive(Clone)]
pub struct DbState {
    pub pool: PgPool
}


pub struct DBController {}

impl DBController {

    pub fn new() -> Self { DBController {  } }

    pub async fn get_pg_pool(self, database_url: String) -> Arc<DbState> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&database_url)
            .await
            .expect("Failed to create database connection pool");

        let _ = self.run_migrations_from_path(&pool)
            .await;

        Arc::new(DbState { pool })
    }

    async fn run_migrations_from_path(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let migrator = Migrator::new(PathMigration::new("./migrations")).await?;
        migrator.run(pool).await?;
        Ok(())
    }

}