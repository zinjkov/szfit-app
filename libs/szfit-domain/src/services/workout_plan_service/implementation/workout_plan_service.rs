use std::sync::Arc;
use async_trait::async_trait;
use dill::component;
use crate::aggregate::WorkoutPlan;
use crate::entity::Id;
use crate::repositories::IWorkoutPlanRepository;
use crate::services::error::ServiceResult;
use crate::services::IWorkoutPlanService;

#[component]
pub struct WorkoutPlanService {
    repo: Arc<dyn IWorkoutPlanRepository>,
}

impl WorkoutPlanService {
    pub fn new(repo: Arc<dyn IWorkoutPlanRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl IWorkoutPlanService for WorkoutPlanService {
    async fn get_workout_plan(&self, workout_id: Id) -> ServiceResult<WorkoutPlan> {
        Ok(self.repo.find_by_id(workout_id).await?)
    }

    async fn set_exercises(&self, workout_id: Id, exercise_ids: Vec<Id>) -> ServiceResult<WorkoutPlan> {
        Ok(self.repo.add_exercises(workout_id, exercise_ids).await?)
    }
}