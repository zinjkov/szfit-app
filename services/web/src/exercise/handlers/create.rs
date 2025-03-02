use crate::{
    app_response::{IntoJsonResult, JsonResult},
    exercise::{
        controllers::exercise_controller::ExerciseController,
        exercise_dto::{ExerciseDto, ExerciseForCreate},
    },
};
use axum::Json;

#[utoipa::path(
    post,
    path = "/exercise",
    responses(
        (status = 200, description = "Getting workout list")
    )
)]
pub async fn create_exercise(
    exercise_controller: ExerciseController,
    Json(exercise_for_create): Json<ExerciseForCreate>,
) -> JsonResult<ExerciseDto> {
    log::debug!("Creating exercise");
    exercise_controller
        .create(exercise_for_create)
        .await
        .into_json()
}
