use crate::entity::Id;

#[derive(Debug, Clone)]
pub struct Workout {
    pub id: Id,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct WorkoutWithOptionId {
    pub id: Option<Id>,
    pub name: String,
}

impl Workout {
    pub fn new(id: Id, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}
