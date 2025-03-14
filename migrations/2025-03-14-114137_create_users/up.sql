-- Your SQL goes here
CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        email VARCHAR(320) UNIQUE NOT NULL,
        password_hash VARCHAR(72) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    );