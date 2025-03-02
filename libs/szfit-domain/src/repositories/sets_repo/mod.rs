use crate::repositories::sets_repo::implementation::postgresql_repo::sets_postgres_repo::PostgresqlSetsRepository;
use crate::repositories::sets_repo::sets_repository::ISetsRepository;
use dill::CatalogBuilder;

mod implementation;
pub mod sets_repository;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder
        .add::<PostgresqlSetsRepository>()
        .bind::<dyn ISetsRepository, PostgresqlSetsRepository>();
}
