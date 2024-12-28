use chrono::{NaiveDateTime};
use crate::entity::Id;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub telegram_id: Id,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn new(id: Id, telegram_id: Id, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            telegram_id,
            created_at
        }
    }
}