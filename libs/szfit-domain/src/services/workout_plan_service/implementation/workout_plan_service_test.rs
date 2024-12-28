use crate::aggregate::WorkoutPlan;
use crate::entity::{Exercise, Id, Workout};
use crate::repositories::error::RepoResult;
use crate::repositories::IWorkoutPlanRepository;
use crate::services::workout_plan_service::implementation::workout_plan_service::WorkoutPlanService;
use crate::services::IWorkoutPlanService;
use async_trait::async_trait;
use mockall::mock;
use std::sync::Arc;

mock! {
    MockWorkoutPlanRepository {}

    #[async_trait]
    impl IWorkoutPlanRepository for MockWorkoutPlanRepository {
        async fn list_with_limit_offset(&self, user_id: Id, limit: usize, offset: usize) -> RepoResult<Vec<Workout>>;

        async fn create(&self, user_id: Id, workout_name: String) -> RepoResult<Workout>;

        async fn delete(&self, workout_id: Id) -> RepoResult<()>;

        async fn update(&self, workout_id: Id, workout_name: String) -> RepoResult<Workout>;

        async fn find_by_id(&self, workout_id: Id) -> RepoResult<WorkoutPlan>;

        async fn add_exercises(&self, workout_id: Id, exercise_ids: Vec<Id>) -> RepoResult<WorkoutPlan>;

        async fn exercises_for_workout(&self, workout_id: Id) -> RepoResult<Vec<Exercise>>;
    }
}

fn prepare_service(mock: MockMockWorkoutPlanRepository) -> impl IWorkoutPlanService {
    WorkoutPlanService::new(Arc::new(mock))
}
#[tokio::test]
async fn test_get_workout_plan() {
    let mut mock = MockMockWorkoutPlanRepository::new();
    mock.expect_find_by_id()
        .returning(|id| Ok(WorkoutPlan {
            workout_id: id,
            workout_name: "test 1".to_string(),
            exercise_list: vec![],
        }));

    let service = prepare_service(mock);
    let res = service.get_workout_plan(Id(1)).await.unwrap();

    assert_eq!(res.workout_id, Id(1));
    assert_eq!(res.workout_name, "test 1");
    assert_eq!(res.exercise_list, vec![]);
}

#[tokio::test]
async fn test_set_exercises() {
    let mut mock = MockMockWorkoutPlanRepository::new();
    mock.expect_add_exercises()
        .returning(|id, ex_ids|
            Ok(WorkoutPlan {
                workout_id: id,
                workout_name: "test 1".to_string(),
                exercise_list: ex_ids
                    .into_iter()
                    .map(|ex_id| Exercise::new(ex_id, format!("exercise {}", *ex_id)))
                    .collect(),
            })
        );

    let service = prepare_service(mock);
    let res = service.set_exercises(Id(1), vec![Id(1)])
        .await
        .unwrap();

    assert_eq!(res.workout_id, Id(1));
    assert_eq!(res.workout_name, "test 1");
    assert_eq!(res.exercise_list.len(), 1);

    let ex = &res.exercise_list[0];
    assert_eq!(ex.id, Id(1));
    assert_eq!(ex.name, "exercise 1");
}