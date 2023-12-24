CREATE TABLE users (
    id            uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_name     VARCHAR(255) UNIQUE NOT NULL,
    email         VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT
);