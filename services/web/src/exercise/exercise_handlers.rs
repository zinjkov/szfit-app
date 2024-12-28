use crate::app_response::JsonResult;
use crate::auth::jwt::ExctractAuthClaims;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use dill::Catalog;
use serde::{Deserialize, Serialize};
use szfit_domain::entity::{Exercise, Id};
use szfit_domain::repositories::{IExerciseRepository};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct ExerciseDto {
    id: Option<Id>,
    name: String,
}

#[derive(Deserialize)]
pub(super) struct ExerciseForCreate {
    name: String,
}

impl From<Exercise> for ExerciseDto {
    fn from(value: Exercise) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
        }
    }
}

impl Into<szfit_domain::repositories::ExerciseForCreate> for ExerciseForCreate {
    fn into(self) -> szfit_domain::repositories::ExerciseForCreate {
        szfit_domain::repositories::ExerciseForCreate {
            name: self.name,
        }
    }
}

#[utoipa::path(
    get,
    path = "/exercise",
    responses(
        (status = 200, description = "Getting workout list")
    )
)]
pub(super) async fn exercise_list(State(catalog): State<Catalog>,
                                  _auth_claims: ExctractAuthClaims)
                                  -> JsonResult<Vec<ExerciseDto>> {
    log::debug!("Getting exercise list");
    let exercise_repo = catalog.get_one::<dyn IExerciseRepository>()?;
    let exercises =  exercise_repo.list_default().await?;

    Ok((StatusCode::OK, Json(exercises.into_iter().map(ExerciseDto::from).collect())))
}


#[utoipa::path(
    post,
    path = "/exercise",
    responses(
        (status = 200, description = "Getting workout list")
    )
)]
pub(super) async fn create_exercise(State(catalog): State<Catalog>,
                                    Json(exercise_for_create): Json<ExerciseForCreate>)
                                    -> JsonResult<ExerciseDto> {
    let exercise_repo = catalog.get_one::<dyn IExerciseRepository>().unwrap();
    log::debug!("Creating exercise {}", exercise_for_create.name);
    let result: ExerciseDto = exercise_repo
        .create(exercise_for_create.into())
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(result)))
}