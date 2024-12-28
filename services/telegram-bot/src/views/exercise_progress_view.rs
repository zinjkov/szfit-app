use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use szfit_domain::entity::Sets;

pub fn exercise_progress_view(exercise_name: String, max_set: Option<Sets>) -> (String, InlineKeyboardMarkup) {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let mut text = format!("Начато упраженение {}\nВводите результаты в формате \"kg reps\"", exercise_name);
    if let Some(sets) = max_set {
        text.push_str(&format!("\n последний результат: {} кг на {} повторений", sets.weight_kg, sets.reps));
    }

    let callback =
        InlineKeyboardButton::callback(
            "Закончить",
            "stop_exercise");
    keyboard.push(vec![callback]);

    (text, InlineKeyboardMarkup::new(keyboard))
}