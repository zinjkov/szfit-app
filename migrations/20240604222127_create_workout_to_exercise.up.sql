-- Add up migration script here
DROP TABLE IF EXISTS workout_to_exercise;
CREATE TABLE workout_to_exercise
(
    id              bigserial PRIMARY KEY,
    workout_plan_id bigint references workout_plan (id) ON DELETE CASCADE,
    exercise_id     bigint references exercise (id) ON DELETE CASCADE
);