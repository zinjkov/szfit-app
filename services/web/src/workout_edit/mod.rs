use crate::workout_edit::workout_edit::update_exercise_into_workout;
use axum::routing::put;
use axum::Router;
use dill::Catalog;

mod workout_edit;

pub fn router() -> Router<Catalog> {
    Router::new()
        .route("/workout-edit", put(update_exercise_into_workout))
}