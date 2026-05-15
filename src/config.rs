use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT").expect("PORT must be set");

        Config {
            database_url: database_url.to_string(),
            port: port.to_string(),
        }
    }
}
