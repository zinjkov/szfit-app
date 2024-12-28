use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use szfit_domain::entity::Id;

pub struct ExerciseViewModel {
    pub id: Id,
    pub name: String,
    pub checked: bool,
}

pub fn workout_progress_view(exercise_list: Vec<ExerciseViewModel>) -> (String, InlineKeyboardMarkup) {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let text = "Список ураженений для тренировки: \n".to_string();

    for exercise in exercise_list {
        let text = if exercise.checked {
            exercise.name + " ✅"
        } else {
            exercise.name
        };
        let callback =
            InlineKeyboardButton::callback(
                text,
                serde_json::to_string(&exercise.id.0).unwrap());
        keyboard.push(vec![callback]);
    }

    let callback =
        InlineKeyboardButton::callback(
            "Закончить тренировку",
            "stop_workout");

    keyboard.push(vec![callback]);

    (text, InlineKeyboardMarkup::new(keyboard))
}