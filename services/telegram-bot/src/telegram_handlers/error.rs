use dill::InjectionError;
use szfit_domain::repositories::error::RepositoryError;
use teloxide::dispatching::dialogue::InMemStorageError;

use szfit_domain::services::error::ServiceError;

use crate::sender::error::SenderError;

#[derive(thiserror::Error, Default, Debug)]
pub enum HandlerError {
    #[error("Context injection error: {0}")]
    ContextInjection(#[from] InjectionError),
    #[error("Empty user state")]
    EmptyUserState,
    #[error("Empty callback data")]
    EmptyCallbackData,
    #[error("Wrong arguments")]
    WrongArgs,
    #[error("Wrong state")]
    WrongState,
    #[error("Callback not found")]
    CallbackNotFound,
    #[error("In-memory storage error: {0}")]
    InMemoryStorageError(#[from] InMemStorageError),
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),
    #[error("Telegram send error: {0}")]
    TelegramSendError(#[from] SenderError),
    #[error("Serde parse error: {0}")]
    SerdeParseError(#[from] serde_json::Error),
    #[default]
    #[error("Other error")]
    Other,
}

pub type HandlerResult<T> = Result<T, HandlerError>;
