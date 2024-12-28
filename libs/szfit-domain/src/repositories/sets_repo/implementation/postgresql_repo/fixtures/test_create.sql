INSERT INTO users(id, telegram_id) VALUES (1, 11);
INSERT INTO workout_plan (id, name, user_id) VALUES (1, 'plan 1', 1);
INSERT INTO exercise(id, name) VALUES (1, 'exercise 1');
INSERT INTO workout_to_exercise(id, workout_plan_id, exercise_id) VALUES (1, 1,1);
INSERT INTO training(id, name, workout_plan_id, user_id) VALUES (1, 'training 1', 1, 1);


