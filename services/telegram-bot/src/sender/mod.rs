use async_trait::async_trait;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;

use error::*;

pub mod error;

pub async fn send_text(bot: &Bot, chat_id: ChatId, text: String) -> SenderResult<Message> {
    log::info!("send_text with text: {:?}", text,);
    Ok(bot
        .send_message(chat_id, text)
        .await?)
}

pub async fn send_text_with_button(bot: &Bot, chat_id: ChatId, text: String, keyboard: InlineKeyboardMarkup) -> SenderResult<Message> {
    log::info!("send_text_with_button with text: {:?}. keyboard:{:?}", text, keyboard);
    Ok(bot
        .send_message(chat_id, text)
        .reply_markup(keyboard)
        .await?)
}

pub struct TelegramReply {
    text: String,
    keyboard: Option<InlineKeyboardMarkup>,
}

#[async_trait]
pub trait TelegramSend {
    async fn send_message(self, bot: &Bot, chat_id: ChatId) -> SenderResult<Message>;
}

#[async_trait]
impl TelegramSend for TelegramReply {
    async fn send_message(self, bot: &Bot, chat_id: ChatId) -> SenderResult<Message> {
        Ok(match self.keyboard {
            None => { send_text(bot, chat_id, self.text).await? }
            Some(keyboard) => { send_text_with_button(bot, chat_id, self.text, keyboard).await? }
        })
    }
}
pub trait IntoTelegramReply: Sized {
    fn into_reply(self) -> TelegramReply;
}

impl IntoTelegramReply for (String, InlineKeyboardMarkup) {
    fn into_reply(self) -> TelegramReply {
        TelegramReply {
            text: self.0,
            keyboard: Some(self.1),
        }
    }
}

impl IntoTelegramReply for (String,) {
    fn into_reply(self) -> TelegramReply {
        TelegramReply {
            text: self.0,
            keyboard: None,
        }
    }
}

impl IntoTelegramReply for String {
    fn into_reply(self) -> TelegramReply {
        TelegramReply {
            text: self,
            keyboard: None,
        }
    }
}