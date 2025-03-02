use dill::InjectionError;
use szfit_domain::repositories::error::RepositoryError;
use teloxide::dispatching::dialogue::InMemStorageError;

use szfit_domain::services::error::ServiceError;

use crate::sender::error::SenderError;

#[derive(thiserror::Error, derive_more::Display, Default, Debug)]
pub enum HandlerError {
    ContextInjection(#[from] InjectionError),
    EmptyUserState,
    EmptyCallbackData,
    WrongArgs,
    WrongState,
    CallbackNotFound,
    InMemoryStorageError(#[from] InMemStorageError),
    RepositoryError(#[from] RepositoryError),
    ServiceError(#[from] ServiceError),
    TelegramSendError(#[from] SenderError),
    SerdeParseError(#[from] serde_json::Error),
    #[default]
    Other,
}

pub type HandlerResult<T> = Result<T, HandlerError>;
