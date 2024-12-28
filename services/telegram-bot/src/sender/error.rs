use teloxide::RequestError;

#[derive(thiserror::Error, derive_more::Display, Default, Debug)]
pub enum SenderError {
    SendRequestError(#[from] RequestError),
    #[default]
    Other
}

pub type SenderResult<T> = std::result::Result<T, SenderError>;