use async_trait::async_trait;
use crate::sender::TelegramReply;
use crate::telegram_handlers::callback_handler::CallbackHandlerProcessor;
use crate::telegram_handlers::error::HandlerResult;

#[async_trait]
pub trait CallbackHandler: Sync + Send {
    async fn apply(&self, chp: &CallbackHandlerProcessor) -> HandlerResult<TelegramReply>;
}
