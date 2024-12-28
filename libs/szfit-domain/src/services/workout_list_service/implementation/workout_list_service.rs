use crate::entity::{Exercise, Id, Workout};
use crate::repositories::IWorkoutPlanRepository;
use crate::services::error::ServiceResult;
use dill::component;
use std::sync::Arc;
use async_trait::async_trait;
use crate::services::workout_list_service::workout_list_service::IWorkoutListService;

#[component]
pub struct WorkoutListService {
    repo: Arc<dyn IWorkoutPlanRepository>,
}

#[async_trait]
impl IWorkoutListService for WorkoutListService {
    async fn create_workout(&self, user_id: Id, name: String) -> ServiceResult<Workout> {
        Ok(self.repo.create(user_id, name).await?)
    }

    async fn delete_workout(&self, user_id: Id) -> ServiceResult<()> {
        Ok(self.repo.delete(user_id).await?)
    }

    async fn update_workout(&self, workout_id: Id, name: String) -> ServiceResult<Workout> {
        Ok(self.repo.update(workout_id, name).await?)
    }
    async fn list_workout_for_user(&self, user_id: Id) -> ServiceResult<Vec<Workout>> {
        Ok(self.repo.list(user_id).await?)
    }

    async fn list_exercise_for_workout(&self, workout_id: Id) -> ServiceResult<Vec<Exercise>> {
        Ok(self.repo.exercises_for_workout(workout_id).await?)
    }
}