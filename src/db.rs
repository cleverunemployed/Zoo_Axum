
use std::{time::Duration};
use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::migrate::Migrator;
use std::path::Path as PathMigration;



pub struct DBController {}

impl DBController {

    pub fn new() -> Self { DBController {  } }

    pub async fn get_pg_pool(self, database_url: String) -> PgPool {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&database_url)
            .await
            .expect("Failed to create database connection pool");

        let _ = self.run_migrations_from_path(&pool)
            .await;

        pool
    }

    async fn run_migrations_from_path(self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let migrator = Migrator::new(PathMigration::new("./migrations")).await?;
        migrator.run(pool).await?;
        Ok(())
    }

}