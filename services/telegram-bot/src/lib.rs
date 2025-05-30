use crate::{
    sender::error::SenderError, state::UserState,
    telegram_command::TelegramCommand, telegram_handlers::error::HandlerResult,
};
use szfit_domain::configure_catalog;
use teloxide::{
    dispatching::{
        dialogue::InMemStorage, Dispatcher, DpHandlerDescription, HandlerExt,
        UpdateFilterExt,
    },
    dptree,
    dptree::Handler,
    prelude::{CallbackQuery, DependencyMap, Message, Requester, Update},
    utils::command::BotCommands,
    Bot,
};

mod telegram_handlers;
use telegram_handlers::*;
mod sender;
mod state;
mod telegram_command;
mod views;

#[derive(Clone)]
struct TelegramContext {
    db: szfit_domain::store::Db,
}

pub struct TelegramBotService {
    context: TelegramContext,
}

impl TelegramBotService {
    pub fn new(db: szfit_domain::store::Db) -> Self {
        Self { context: TelegramContext { db } }
    }
    fn handlers(&self) -> HandlerType {
        dptree::entry()
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<UserState>, UserState>()
                    .filter_command::<TelegramCommand>()
                    .endpoint(handle_command),
            )
            .branch(
                Update::filter_callback_query()
                    .enter_dialogue::<CallbackQuery, InMemStorage<UserState>, UserState>()
                    .endpoint(handle_callback),
            )
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<UserState>, UserState>()
                    .endpoint(handle_message),
            )
    }
    pub async fn start(self) -> HandlerResult<()> {
        let bot = Bot::from_env();
        let state = InMemStorage::<UserState>::new();
        let mut catalog_builder = configure_catalog();
        catalog_builder.add_value(bot.clone());
        catalog_builder.add_value(self.context.db.clone());

        let _ = bot
            .set_my_commands(TelegramCommand::bot_commands())
            .await;

        Dispatcher::builder(bot, self.handlers())
            // .enable_ctrlc_handler()
            .dependencies(dptree::deps![
                self.context,
                state,
                catalog_builder.build()
            ])
            .build()
            .dispatch()
            .await;
        Ok(())
    }
}

type TelegramResult<T> = Result<T, TelegramError>;

#[derive(thiserror::Error, derive_more::Display, Debug)]
pub enum TelegramError {
    HandleError(#[from] error::HandlerError),
    SenderError(#[from] SenderError),
    Other,
}

pub type DtreeHandlerResult = Result<(), TelegramError>;
pub type HandlerType =
    Handler<'static, DependencyMap, DtreeHandlerResult, DpHandlerDescription>;
