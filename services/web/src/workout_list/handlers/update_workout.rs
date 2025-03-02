use crate::{
    app_response::{IntoJsonResult, JsonResult},
    auth::jwt::ExctractAuthClaims,
    workout_list::{
        controllers::workout_controller::WorkoutController,
        workout_dto::{WorkoutDto, WorkoutForUpdate},
    },
};
use axum::Json;

#[utoipa::path(
    put,
    path = "/workout",
    responses(
        (status = 200, description = "Update a workout", body = WorkoutDto)
    ),
    params(
        ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
)]
pub async fn update_workout(
    workout_controller: WorkoutController, _: ExctractAuthClaims,
    Json(workout_for_update): Json<WorkoutForUpdate>,
) -> JsonResult<WorkoutDto> {
    workout_controller
        .update_workout(workout_for_update)
        .await
        .into_json()
}
