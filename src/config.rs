use dotenvy::dotenv;
use std::env;

pub struct Config {
    database_url: String,
    port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT").expect("PORT must be set");


        Config(
            database_url.to_string(),
            port.to_string()
        )
    }
}