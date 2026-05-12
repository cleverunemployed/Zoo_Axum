-- Add migration script here
CREATE TABLE IF NOT EXISTS animals (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    category VARCHAR(50) NOT NULL,
    health SMALLINT NOT NULL CHECK (health >= 0 AND health <= 100),
    satiety SMALLINT NOT NULL CHECK (satiety >= 0 AND satiety <= 100)
)