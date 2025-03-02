use crate::workout_edit::handlers::update_exercise_into_workout;
use axum::{routing::put, Router};
use dill::Catalog;

mod controllers;
mod handlers;
mod workout_dto;

pub fn router() -> Router<Catalog> {
    Router::new().route("/workout-edit", put(update_exercise_into_workout))
}
