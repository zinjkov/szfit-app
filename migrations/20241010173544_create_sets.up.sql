-- Add up migration script here
DROP TABLE IF EXISTS sets;
CREATE TABLE sets
(
    id          bigserial PRIMARY KEY,
    weight_kg   real    NOT NULL,
    reps        integer NOT NULL,
    exercise_id bigint REFERENCES exercise (id)  NOT NULL,
    user_id     bigint REFERENCES users (id)  NOT NULL,
    training_id bigint REFERENCES training (id)  NOT NULL,
    created_at  timestamp DEFAULT NOW() NOT NULL
);