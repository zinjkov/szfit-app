use crate::entity::{Id, Training};
use crate::repositories::TrainingForCreate;
use crate::services::error::ServiceResult;
use async_trait::async_trait;


pub struct StartTrainingArgs {
    pub user_id: Id,
    pub name: Option<String>,
    pub workout_plan_id: Id,
}
impl StartTrainingArgs {
    pub fn new(user_id: Id, workout_plan_id: Id, name: Option<String>) -> Self {
        Self {
            user_id,
            name,
            workout_plan_id,
        }
    }
}
impl Into<TrainingForCreate> for StartTrainingArgs {
    fn into(self) -> TrainingForCreate {
        TrainingForCreate {
            name: self.name,
            user_id: self.user_id,
            workout_plan_id: self.workout_plan_id,
        }
    }
}

#[async_trait]
pub trait ITrainingService: Send + Sync {
    async fn start_training(&self, sta: StartTrainingArgs) -> ServiceResult<Training>;
    async fn finish_training(&self, training_id: Id) -> ServiceResult<Training>;
}