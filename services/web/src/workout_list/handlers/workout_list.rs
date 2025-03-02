use crate::{
    app_response::{IntoJsonResult, JsonResult},
    auth::jwt::ExctractAuthClaims,
    workout_list::{
        controllers::workout_controller::WorkoutController,
        workout_dto::WorkoutDto,
    },
};

#[utoipa::path(
    get,
    path = "/workout",
    responses(
        (status = 200, description = "Getting workout list")
    )
)]
pub async fn workout_list(
    controller: WorkoutController, auth_claims: ExctractAuthClaims,
) -> JsonResult<Vec<WorkoutDto>> {
    controller
        .list_workout_for_user(auth_claims.user_id)
        .await
        .into_json()
}
