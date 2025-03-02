use crate::services::workout_plan_service::implementation::workout_plan_service::WorkoutPlanService;
use crate::services::IWorkoutPlanService;
use dill::CatalogBuilder;

mod implementation;
pub mod workout_plan_service;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder
        .add::<WorkoutPlanService>()
        .bind::<dyn IWorkoutPlanService, WorkoutPlanService>();
}
