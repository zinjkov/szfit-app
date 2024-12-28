use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use szfit_domain::entity::Workout;

pub fn workout_list_view(workout_list: Vec<Workout>) -> (String, InlineKeyboardMarkup) {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    for workout in workout_list {
        let callback =
            InlineKeyboardButton::callback(
                workout.name,
                serde_json::to_string(&workout.id.0).unwrap());
        keyboard.push(vec![callback]);
    }
    ("Твои программы тернировок".to_string(), InlineKeyboardMarkup::new(keyboard))
}