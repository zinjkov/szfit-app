-- Add up migration script here
DROP TABLE IF EXISTS exercise;
CREATE TABLE exercise
(
    id   bigserial PRIMARY KEY,
    name TEXT NOT NULL
);