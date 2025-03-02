use crate::{
    app_response::JsonResult,
    auth::jwt::ExctractAuthClaims,
    workout_list::{
        controllers::workout_controller::WorkoutController,
        workout_dto::WorkoutForDelete,
    },
};
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use szfit_domain::services::IWorkoutListService;

#[utoipa::path(
    delete,
    path = "/workout",
    responses(
        (status = 200, description = "Delete a workout")
    )
)]
pub async fn delete_workout(
    workout_controller: WorkoutController, _auth_claims: ExctractAuthClaims,
    Json(workout_for_delete): Json<WorkoutForDelete>,
) -> JsonResult<Value> {
    workout_controller
        .delete_workout(workout_for_delete)
        .await?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "ok",
        })),
    ))
}
