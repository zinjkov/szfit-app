use dill::CatalogBuilder;
use implementation::postgresql_repo::workout_plan_postgres_repo::PostgresqlWorkoutPlanRepo;
use workout_plan_repository::IWorkoutPlanRepository;

mod implementation;
pub mod workout_plan_repository;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder
        .add::<PostgresqlWorkoutPlanRepo>()
        .bind::<dyn IWorkoutPlanRepository, PostgresqlWorkoutPlanRepo>();
}
