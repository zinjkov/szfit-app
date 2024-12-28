mod workout_list;

use axum::routing::{delete, get, post, put};
use axum::Router;
use dill::Catalog;
use utoipa::OpenApi;


pub fn router() -> Router<Catalog> {
    Router::new()
        .route("/workout", get(workout_list::workout_list))
        .route("/workout", post(workout_list::create_workout))
        .route("/workout", delete(workout_list::delete_workout))
        .route("/workout", put(workout_list::update_workout))
}

use workout_list::*;
#[derive(OpenApi)]
#[openapi(
    paths(workout_list, create_workout, delete_workout, update_workout),
    components(schemas(WorkoutForUpdate, WorkoutDto))
)]
pub struct ApiDocWorkout;