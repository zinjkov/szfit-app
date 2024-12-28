use crate::repositories::exercise_repo::implementation::postgresql_repo::exercise_postgres_repo::PostgresqlExerciseRepository;
use crate::repositories::IExerciseRepository;
use dill::CatalogBuilder;

mod implementation;
pub mod exercise_repository;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<PostgresqlExerciseRepository>()
        .bind::<dyn IExerciseRepository, PostgresqlExerciseRepository>();
}