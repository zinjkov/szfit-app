use std::sync::Arc;
use chrono::{Utc};
use dill::component;
use crate::entity::{Id, Training};
use crate::repositories::{ITrainingRepository, TrainingForCreate, TrainingForUpdate};
use crate::services::error::ServiceResult;

#[component]
pub struct TrainingService {
    training_repo: Arc<dyn ITrainingRepository>
}

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

impl TrainingService {
    pub async fn start_training(&self, tfc: impl Into<TrainingForCreate>) -> ServiceResult<Training> {
        Ok(self.training_repo.create(tfc.into()).await?)
    }

    pub async fn finish_training(&self, training_id: Id) -> ServiceResult<Training> {
        Ok(self.training_repo.update(training_id, TrainingForUpdate {
            finished_at: Some(Utc::now().naive_utc()),
            ..Default::default()
        }).await?)
    }
}