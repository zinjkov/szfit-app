
#[derive(thiserror::Error, derive_more::Display, Debug, Default)]
pub enum RepositoryError {
    DbError(#[from] sqlx::Error),
    EntityNotFound,
    #[default]
    Other
}

pub type RepoResult<T> = std::result::Result<T, RepositoryError>;