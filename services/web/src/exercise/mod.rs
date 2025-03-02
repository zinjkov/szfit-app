mod controllers;
mod exercise_dto;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use dill::Catalog;

pub fn router() -> Router<Catalog> {
    Router::new()
        .route("/exercise", get(handlers::exercise_list))
        .route("/exercise", post(handlers::create_exercise))
}
