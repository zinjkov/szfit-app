mod controllers;
mod handlers;
mod workout_dto;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dill::Catalog;
use utoipa::OpenApi;

pub fn router() -> Router<Catalog> {
    Router::new()
        .route("/workout", get(handlers::workout_list))
        .route("/workout", post(handlers::create_workout))
        .route("/workout", delete(handlers::delete_workout))
        .route("/workout", put(handlers::update_workout))
}

use workout_dto::*;
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::workout_list,
        handlers::create_workout,
        handlers::delete_workout,
        handlers::update_workout
    ),
    components(schemas(WorkoutForUpdate, WorkoutDto))
)]
pub struct ApiDocWorkout;
