INSERT INTO users(id, telegram_id) VALUES (1, 11);
INSERT INTO workout_plan (id, name, user_id) VALUES (1, 'plan 1', 1);
INSERT INTO training (id, name, workout_plan_id, user_id)
    VALUES (1, 'training 1', 1, 1);
INSERT INTO training (id, name, workout_plan_id, user_id)
    VALUES (2, 'training 2', 1, 1);



