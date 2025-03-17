-- Your SQL goes here
CREATE TABLE
    web_instances (
        id SERIAL PRIMARY KEY,
        name VARCHAR(64) UNIQUE NOT NULL,
        description VARCHAR(256) NOT NULL,
        protocol VARCHAR(8) NOT NULL DEFAULT 'http' CHECK(protocol IN ('http', 'https')),
        host VARCHAR(64) NOT NULL,
        port INTEGER NOT NULL DEFAULT 80 CHECK(port > 0 AND port < 65536),
        path VARCHAR(256) NOT NULL DEFAULT '/',
        module_id INTEGER NOT NULL REFERENCES modules(id),
        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    );