-- Add up migration script here

DROP TABLE IF EXISTS training;
CREATE TABLE training
(
    id              bigserial PRIMARY KEY,
    name            VARCHAR(255) NOT NULL DEFAULT CURRENT_DATE,
    workout_plan_id bigint       NOT NULL REFERENCES workout_plan (id),
    user_id         bigint       NOT NULL REFERENCES users (id),
    created_at      timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at     timestamp
);