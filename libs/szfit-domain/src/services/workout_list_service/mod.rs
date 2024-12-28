use crate::services::workout_list_service::implementation::workout_list_service::WorkoutListService;
use crate::services::workout_list_service::workout_list_service::IWorkoutListService;
use dill::CatalogBuilder;

pub mod workout_list_service;
mod implementation;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<WorkoutListService>()
        .bind::<dyn IWorkoutListService, WorkoutListService>();
}