use crate::entity::Id;
use chrono::NaiveDateTime;

#[derive(Debug, Default)]
pub struct Training {
    pub id: Id,
    pub name: String,
    pub workout_plan_id: Id,
    pub user_id: Id,
    pub created_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}