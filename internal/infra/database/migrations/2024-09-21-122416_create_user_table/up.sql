CREATE TABLE IF NOT EXISTS users
(
    id           SERIAL PRIMARY KEY,
    name         TEXT NOT NULL,
    email        TEXT NOT NULL,
    password     TEXT NOT NULL,
    created_date TIMESTAMP NOT NULL,
    updated_date TIMESTAMP NOT NULL,
    deleted_date TIMESTAMP NULL
);
