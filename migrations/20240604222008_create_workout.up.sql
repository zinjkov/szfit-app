-- Add up migration script here
DROP TABLE if exists workout_plan cascade;
CREATE TABLE workout_plan
(
    id      bigserial PRIMARY KEY,
    name    VARCHAR(255) NOT NULL,
    user_id bigint REFERENCES users (id)
);