use crate::{
    entity::{Id, Training},
    repositories::{ITrainingRepository, TrainingForUpdate},
    services::{error::ServiceResult, ITrainingService, StartTrainingArgs},
};
use async_trait::async_trait;
use chrono::Utc;
use dill::component;
use std::sync::Arc;

#[component]
pub struct TrainingService {
    training_repo: Arc<dyn ITrainingRepository>,
}

impl TrainingService {
    #[allow(unused)]
    pub fn new(training_repo: Arc<dyn ITrainingRepository>) -> Self {
        Self { training_repo }
    }
}

#[async_trait]
impl ITrainingService for TrainingService {
    async fn start_training(
        &self, sta: StartTrainingArgs,
    ) -> ServiceResult<Training> {
        Ok(self.training_repo.create(sta.into()).await?)
    }

    async fn finish_training(
        &self, training_id: Id,
    ) -> ServiceResult<Training> {
        Ok(self
            .training_repo
            .update(
                training_id,
                TrainingForUpdate {
                    finished_at: Some(Utc::now().naive_utc()),
                    ..Default::default()
                },
            )
            .await?)
    }
}
