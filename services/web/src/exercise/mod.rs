mod exercise_handlers;

use axum::routing::{get, post};
use axum::Router;
use dill::Catalog;

pub fn router() -> Router<Catalog> {
    Router::new()
        .route("/exercise", get(exercise_handlers::exercise_list))
        .route("/exercise", post(exercise_handlers::create_exercise))
}