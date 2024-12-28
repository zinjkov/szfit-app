use dill::CatalogBuilder;
use crate::services::IWorkoutPlanService;
use crate::services::workout_plan_service::implementation::workout_plan_service::WorkoutPlanService;

pub mod workout_plan_service;
mod implementation;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<WorkoutPlanService>()
        .bind::<dyn IWorkoutPlanService, WorkoutPlanService>();
}