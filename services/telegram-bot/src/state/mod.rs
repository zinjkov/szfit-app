use std::sync::Arc;

use dill::Catalog;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;
use teloxide::types::ChatId;
use tokio::sync::Mutex;

use szfit_domain::entity::User;
use szfit_domain::services;
use szfit_domain::usecases::WorkoutUsecase;

use crate::telegram_handlers::error::*;

#[derive(Clone, Default, Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub enum Screen {
    #[default]
    Start,
    WorkoutList,
    ExerciseList,
    WorkoutInProgress,
    ExerciseInProgress,
}

#[derive(Clone, Default)]
pub struct UserState {
    pub state: Screen,
    pub user: Option<User>,
    pub workout_usecase: Option<Arc<Mutex<WorkoutUsecase>>>,
}

pub type UserDialogue = Dialogue<UserState, InMemStorage<UserState>>;

pub async fn auth(catalog: &Catalog, chat_id: ChatId, user_dialog: &UserDialogue) -> HandlerResult<User> {
    let mut user_state = user_dialog.get().await?.ok_or(HandlerError::EmptyUserState)?;
    if user_state.user.is_none() {
        let user = {
            let service = catalog.get_one::<services::AuthService>()?;
            service
                .user_or_create(chat_id.0.into())
                .await?
        };
        user_state.user = Some(user.clone());
        user_dialog.update(user_state.clone()).await?;
        Ok(user)
    } else {
        Ok(user_state.user.unwrap().clone())
    }
}