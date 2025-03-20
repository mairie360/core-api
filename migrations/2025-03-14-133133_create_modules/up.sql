-- Your SQL goes here
CREATE TABLE
    modules (
        id SERIAL PRIMARY KEY,
        name VARCHAR(64) UNIQUE NOT NULL,
        full_name VARCHAR(64) NOT NULL,
        description VARCHAR(256) NOT NULL,
        api_url VARCHAR(2048) NOT NULL,
        web_url VARCHAR(2048) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
    );