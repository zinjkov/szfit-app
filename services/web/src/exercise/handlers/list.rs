use crate::{
    app_response::{IntoJsonResult, JsonResult},
    auth::jwt::ExctractAuthClaims,
    exercise::{
        controllers::exercise_controller::ExerciseController,
        exercise_dto::ExerciseDto,
    },
};

#[utoipa::path(
    get,
    path = "/exercise",
    responses(
        (status = 200, description = "Getting workout list")
    )
)]
pub async fn exercise_list(
    exercise_controller: ExerciseController, _auth_claims: ExctractAuthClaims,
) -> JsonResult<Vec<ExerciseDto>> {
    log::debug!("Getting exercise list");
    exercise_controller.list(1000).await.into_json()
}
