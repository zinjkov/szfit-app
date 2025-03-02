use crate::entity::{Id, Training};
use crate::repositories::error::RepoResult;
use crate::repositories::{
    ITrainingRepository, TrainingForCreate, TrainingForUpdate,
};
use crate::services::training_service::implementation::training_service::TrainingService;
use crate::services::{ITrainingService, StartTrainingArgs};
use async_trait::async_trait;
use chrono::Utc;
use mockall::mock;
use std::sync::Arc;

mock! {
    TrainingRepo {}

    #[async_trait]
    impl ITrainingRepository for TrainingRepo {
        async fn create(&self, tfc: TrainingForCreate) -> RepoResult<Training>;
        async fn list(&self, user_id: Id, limit: usize, offset: usize) -> RepoResult<Vec<Training>>;
        async fn update(&self, training_id: Id, tfu: TrainingForUpdate) -> RepoResult<Training>;
    }
}

fn prepare_service(mock: MockTrainingRepo) -> impl ITrainingService {
    TrainingService::new(Arc::new(mock))
}

//start_training
#[tokio::test]
async fn test_start_training() {
    let mut mock = MockTrainingRepo::new();
    mock.expect_create().returning(|tfc| {
        Ok(Training {
            id: Id(1),
            name: tfc.name.unwrap_or(Utc::now().to_string()),
            workout_plan_id: Id(1),
            user_id: Id(1),
            created_at: Utc::now().naive_utc(),
            finished_at: None,
        })
    });

    let service = prepare_service(mock);
    let res = service
        .start_training(StartTrainingArgs {
            user_id: Id(1),
            name: Some("training 1".to_string()),
            workout_plan_id: Id(1),
        })
        .await
        .expect("could not start training");

    assert_eq!(res.id, Id(1));
    assert_eq!(res.user_id, Id(1));
    assert_eq!(res.name, "training 1");
    assert_eq!(res.workout_plan_id, Id(1));
    assert_eq!(res.finished_at, None);
}

#[tokio::test]
async fn test_finish_training() {
    let mut mock = MockTrainingRepo::new();
    mock.expect_update().returning(|id, update_data| {
        Ok(Training {
            id: id,
            name: "training1".to_string(),
            workout_plan_id: Id(1),
            user_id: Id(1),
            created_at: Utc::now().naive_utc(),
            finished_at: update_data.finished_at,
        })
    });

    let service = prepare_service(mock);
    let res = service
        .finish_training(Id(1))
        .await
        .expect("could not finish training");

    assert_eq!(res.id, Id(1));
    assert!(res.finished_at.is_some());
}

#[tokio::test]
async fn error_test() {}
