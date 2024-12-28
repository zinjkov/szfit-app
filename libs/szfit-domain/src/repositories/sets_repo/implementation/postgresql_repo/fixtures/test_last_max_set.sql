INSERT INTO users(id, telegram_id) VALUES (1, 11);
INSERT INTO workout_plan (id, name, user_id) VALUES (1, 'plan 1', 1);
INSERT INTO exercise(id, name) VALUES (1, 'exercise 1');
INSERT INTO workout_to_exercise(id, workout_plan_id, exercise_id) VALUES (1, 1,1);
INSERT INTO training(id, name, workout_plan_id, user_id) VALUES (1, 'training 1', 1, 1);

INSERT INTO sets(id, weight_kg, reps, exercise_id, user_id, training_id, created_at)
    VALUES (1, 1, 1, 1, 1, 1, '2024-12-28 13:43:21.645959');

INSERT INTO sets(id, weight_kg, reps, exercise_id, user_id, training_id, created_at)
    VALUES (2, 0, 0, 1, 1, 1, '2024-12-28 13:44:21.645959');

INSERT INTO sets(id, weight_kg, reps, exercise_id, user_id, training_id, created_at)
    VALUES (3, 1, 1, 1, 1, 1, '2024-12-28 13:45:21.645959');
