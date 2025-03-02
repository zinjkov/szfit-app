
#[derive(thiserror::Error, Debug, Default)]
pub enum RepositoryError {
    #[error("Ошибка базы данных: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Сущность не найдена")]
    EntityNotFound,
    #[error("Другая ошибка")]
    #[default]
    Other,
}

pub type RepoResult<T> = std::result::Result<T, RepositoryError>;
