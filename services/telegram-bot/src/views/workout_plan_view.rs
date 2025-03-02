use std::fmt;

use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use szfit_domain::aggregate::WorkoutPlan;
use szfit_domain::entity::Id;

#[derive(Serialize, Deserialize, Default)]
pub enum WorkoutPlanCallbackType {
    Start,
    #[default]
    Back,
}

#[derive(Serialize, Deserialize, Default)]
pub struct WorkoutPlanCallbackData {
    pub callback_type: WorkoutPlanCallbackType,
    pub workout_id: Option<i64>,
}

impl WorkoutPlanCallbackData {
    pub fn start(workout_id: Id) -> Self {
        Self {
            callback_type: WorkoutPlanCallbackType::Start,
            workout_id: Some(*workout_id),
        }
    }

    pub fn back() -> Self {
        Self {
            callback_type: WorkoutPlanCallbackType::Back,
            workout_id: None,
        }
    }
}

pub fn workout_plan_view(
    workout_plan: WorkoutPlan,
) -> (String, InlineKeyboardMarkup) {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let mut text = format!(
        "Список ураженений для тренировки {}: \n",
        capitalize(workout_plan.workout_name)
    );

    workout_plan.exercise_list.into_iter().for_each(|ex| {
        use fmt::Write;
        let _ = write!(text, "- {}\n", capitalize(ex.name));
    });

    keyboard.push(vec![
        InlineKeyboardButton::callback(
            "Начать",
            serde_json::to_string(&WorkoutPlanCallbackData::start(
                workout_plan.workout_id,
            ))
            .unwrap(),
        ),
        InlineKeyboardButton::callback(
            "Назад",
            serde_json::to_string(&WorkoutPlanCallbackData::back())
                .unwrap(),
        ),
    ]);

    (text, InlineKeyboardMarkup::new(keyboard))
}

fn capitalize(string: String) -> String {
    let mut chars = string.chars();
    let Some(first) = chars.next() else {
        return String::with_capacity(0);
    };
    first
        .to_uppercase()
        .chain(chars.flat_map(char::to_lowercase))
        .collect()
}
