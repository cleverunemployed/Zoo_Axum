-- Миграция: создание таблиц users и user_animals
-- Дата: 2026-05-14


CREATE TABLE IF NOT EXISTS animals (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    category VARCHAR(50) NOT NULL,
    health SMALLINT NOT NULL CHECK (health >= 0 AND health <= 100),
    satiety SMALLINT NOT NULL CHECK (satiety >= 0 AND satiety <= 100)
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    is_deleted BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS user_animals (
    id SERIAL PRIMARY KEY,
    id_user INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    id_animal INTEGER NOT NULL REFERENCES animals(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_animal UNIQUE (id_user, id_animal)
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email) WHERE is_deleted = false;
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_animals_category ON animals(category);
CREATE INDEX IF NOT EXISTS idx_animals_name ON animals(name);
CREATE INDEX IF NOT EXISTS idx_user_animals_user ON user_animals(id_user);
CREATE INDEX IF NOT EXISTS idx_user_animals_animal ON user_animals(id_animal);

COMMENT ON TABLE animals IS 'Таблица с информацией о животных';
COMMENT ON COLUMN animals.health IS 'Здоровье животного (0-100)';
COMMENT ON COLUMN animals.satiety IS 'Сытость животного (0-100)';
COMMENT ON TABLE users IS 'Таблица с пользователями системы';
COMMENT ON COLUMN users.email IS 'Email пользователя (уникальный)';
COMMENT ON COLUMN users.role IS 'Роль пользователя (user/admin и т.д.)';
COMMENT ON COLUMN users.is_deleted IS 'Флаг мягкого удаления';
COMMENT ON TABLE user_animals IS 'Связующая таблица между пользователями и животными (many-to-many)';
COMMENT ON COLUMN user_animals.id_user IS 'ID пользователя (внешний ключ к users.id)';
COMMENT ON COLUMN user_animals.id_animal IS 'ID животного (внешний ключ к animals.id)';
COMMENT ON COLUMN user_animals.created_at IS 'Дата и время создания связи';