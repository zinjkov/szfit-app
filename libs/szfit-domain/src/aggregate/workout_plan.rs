use crate::entity::{Exercise, Id};

#[derive(Debug, Clone)]
pub struct WorkoutPlan {
    pub workout_id: Id,
    pub workout_name: String,
    pub exercise_list: Vec<Exercise>,
}
