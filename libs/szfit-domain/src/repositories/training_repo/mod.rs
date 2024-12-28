use crate::repositories::training_repo::implementation::postgresql_repo::training_postgres_repo::PostgresqlTrainingRepository;
use crate::repositories::ITrainingRepository;
use dill::CatalogBuilder;

mod implementation;
pub mod training_repository;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<PostgresqlTrainingRepository>()
        .bind::<dyn ITrainingRepository, PostgresqlTrainingRepository>();
}