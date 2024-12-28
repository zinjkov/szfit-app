-- Add up migration script here

DROP TABLE IF EXISTS users;
CREATE TABLE users
(
    id bigserial PRIMARY KEY,
    telegram_id bigint UNIQUE NOT NULL,
    created_at  timestamp     NOT NULL DEFAULT NOW()
);