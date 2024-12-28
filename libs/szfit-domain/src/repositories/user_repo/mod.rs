use crate::repositories::user_repo::implementation::postgresql_repo::user_postgres_repo::PostgresqlUserRepo;
use crate::repositories::IUserRepository;
use dill::CatalogBuilder;

mod implementation;
pub mod user_repository;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<PostgresqlUserRepo>()
        .bind::<dyn IUserRepository, PostgresqlUserRepo>();
}