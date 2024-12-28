use crate::entity::Id;
use chrono::NaiveDateTime;

pub struct Sets {
    pub id: Id,
    pub weight_kg: f64,
    pub reps: i64,
    pub exercise_id: Id,
    pub user_id: Id,
    pub created_at: NaiveDateTime,
    pub training_id: Id,
}