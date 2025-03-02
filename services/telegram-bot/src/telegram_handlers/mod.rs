use dill::Catalog;
use teloxide::prelude::{CallbackQuery, Message};
use teloxide::Bot;

use error::*;

use crate::sender::send_text;
use crate::state::{Screen, UserDialogue};
use crate::telegram_handlers::callback_handler::CallbackHandlerProcessor;
use crate::telegram_handlers::command_handler::CommandHandlerProcessor;
use crate::telegram_handlers::error::HandlerError;
use crate::telegram_handlers::message_handler::MessageHandlerProcessor;
use crate::{TelegramCommand, TelegramResult};

mod callback_handler;
mod command_handler;
pub mod error;
pub mod handlers;
mod message_handler;

pub async fn handle_message(
    bot: Bot,
    ud: UserDialogue,
    msg: Message,
) -> TelegramResult<()> {
    log::info!("handled message telegram");
    Ok(MessageHandlerProcessor::new(bot, msg, ud)
        .process()
        .await
        .inspect_err(|err| {
            log::error!("callback raised error: {err:?}")
        })?)
}

pub async fn handle_callback(
    bot: Bot,
    ud: UserDialogue,
    cq: CallbackQuery,
    cat: Catalog,
) -> TelegramResult<()> {
    log::info!("handled callback telegram");
    let cbot = bot.clone();
    let chat_id = ud.chat_id();
    let result = CallbackHandlerProcessor::new(bot, cq, ud, cat)
        .process()
        .await
        .inspect_err(|err| log::error!("callback raised error: {err:?}"));
    if result.is_err() {
        let _ = send_text(
            &cbot,
            chat_id,
            format!(
                "something went wrong {}",
                result.as_ref().unwrap_err().to_string()
            )
            .to_string(),
        )
        .await;
    }
    Ok(result?)
}

pub async fn handle_command(
    bot: Bot,
    command: TelegramCommand,
    msg: Message,
    ud: UserDialogue,
    cat: Catalog,
) -> TelegramResult<()> {
    log::info!("handled command telegram");
    Ok(CommandHandlerProcessor::new(bot, command, msg, ud, cat)
        .process()
        .await
        .inspect_err(|err| {
            log::error!("callback raised error: {err:?}")
        })?)
}

pub async fn set_state(
    user_dialog: &UserDialogue,
    screen: Screen,
) -> HandlerResult<()> {
    log::info!(
        "Set screen {:?} for user {:?}",
        screen,
        user_dialog.chat_id().0
    );

    let mut user_state =
        user_dialog.get().await?.ok_or(HandlerError::EmptyUserState)?;
    user_state.state = screen;
    user_dialog.update(user_state).await?;
    Ok(())
}
