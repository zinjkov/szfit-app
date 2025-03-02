use crate::{
    app_response::IntoJsonResult,
    auth::jwt::ExctractAuthClaims,
    workout_list::{
        controllers::workout_controller::WorkoutController,
        workout_dto::WorkoutForCreate,
    },
};
use axum::{response::IntoResponse, Json};
use szfit_domain::services::IWorkoutListService;

#[utoipa::path(
    post,
    path = "/workout",
    responses(
        (status = 201, description = "Create a workout")
    )
)]
pub async fn create_workout(
    create_controller: WorkoutController, auth_claims: ExctractAuthClaims,
    Json(workout_for_create): Json<WorkoutForCreate>,
) -> impl IntoResponse {
    create_controller
        .create_workout(auth_claims.user_id, workout_for_create)
        .await
        .into_json()
}
