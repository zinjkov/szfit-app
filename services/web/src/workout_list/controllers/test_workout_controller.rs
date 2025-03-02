use crate::workout_list::controllers::workout_controller::WorkoutController;
use std::sync::Arc;
use szfit_domain::entity::Id;

use crate::workout_list::workout_dto::{
    WorkoutForCreate, WorkoutForDelete, WorkoutForUpdate,
};
use async_trait::async_trait;
use mockall::predicate::eq;
use szfit_domain::{
    entity::Workout,
    services::{
        error::{ServiceError, ServiceResult},
        IWorkoutListService,
    },
};

mockall::mock! {
    pub IWorkoutListService {}

    #[async_trait]
    impl IWorkoutListService for IWorkoutListService {
        async fn create_workout(
            &self,
            user_id: Id,
            name: String,
        ) -> ServiceResult<Workout>;

        async fn delete_workout(&self, user_id: Id) -> ServiceResult<()>;

        async fn update_workout(
            &self,
            workout_id: Id,
            name: String,
        ) -> ServiceResult<Workout>;

        async fn list_workout_for_user(
            &self,
            user_id: Id,
        ) -> ServiceResult<Vec<Workout>>;

        async fn list_exercise_for_workout(
            &self,
            workout_id: Id,
        ) -> ServiceResult<Vec<szfit_domain::entity::Exercise >>;
    }
}
#[tokio::test]
async fn test_list_workout_for_user() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_list_workout_for_user()
        .with(eq(Id(1)))
        .returning(|_| {
            Ok(vec![Workout { id: Id(1), name: "test".to_string() }])
        });

    let controller = WorkoutController::new(Arc::new(mock_service));

    let result = controller.list_workout_for_user(Id(1)).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[should_panic]
async fn test_list_workout_for_user_panic() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_list_workout_for_user()
        .with(eq(Id(1)))
        .returning(|_| Err(ServiceError::Other));

    let controller = WorkoutController::new(Arc::new(mock_service));

    let _ = controller
        .list_workout_for_user(Id(1))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_workout() {
    let mut mock_service = MockIWorkoutListService::new();

    mock_service
        .expect_create_workout()
        .with(eq(Id(1)), eq("test".to_string()))
        .returning(|_, _| Ok(Workout { id: Id(1), name: "test".to_string() }));

    let controller = WorkoutController::new(Arc::new(mock_service));
    let workout_for_create = WorkoutForCreate { name: "test".to_string() };
    let result = controller
        .create_workout(Id(1), workout_for_create)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[should_panic]
async fn test_create_workout_panic() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_create_workout()
        .with(eq(Id(1)), eq("test".to_string()))
        .returning(|_, _| Err(ServiceError::Other));

    let controller = WorkoutController::new(Arc::new(mock_service));
    let workout_for_create = WorkoutForCreate { name: "test".to_string() };
    let _ = controller
        .create_workout(Id(1), workout_for_create)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_delete_workout() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_delete_workout()
        .with(eq(Id(1)))
        .returning(|_| Ok(()));

    let controller = WorkoutController::new(Arc::new(mock_service));
    let workout_for_delete = WorkoutForDelete { id: Id(1) };
    let result = controller
        .delete_workout(workout_for_delete)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[should_panic]
async fn test_delete_workout_panic() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_delete_workout()
        .with(eq(Id(1)))
        .returning(|_| Err(ServiceError::Other));

    let controller = WorkoutController::new(Arc::new(mock_service));

    let _ = controller
        .delete_workout(WorkoutForDelete { id: Id(1) })
        .await
        .unwrap();
}

#[tokio::test]
async fn test_update_workout() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_update_workout()
        .with(eq(Id(1)), eq("test".to_string()))
        .returning(|_, _| Ok(Workout { id: Id(1), name: "test".to_string() }));

    let controller = WorkoutController::new(Arc::new(mock_service));
    let workout_for_update =
        WorkoutForUpdate { id: Id(1), name: "test".to_string() };
    let result = controller
        .update_workout(workout_for_update)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[should_panic]
async fn test_update_workout_panic() {
    let mut mock_service = MockIWorkoutListService::new();
    mock_service
        .expect_update_workout()
        .with(eq(Id(1)), eq("test".to_string()))
        .returning(|_, _| Err(ServiceError::Other));

    let controller = WorkoutController::new(Arc::new(mock_service));
    let workout_for_update =
        WorkoutForUpdate { id: Id(1), name: "test".to_string() };

    let _ = controller
        .update_workout(workout_for_update)
        .await
        .unwrap();
}
