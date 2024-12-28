use crate::auth::jwt::ExctractAuthClaims;
use axum::extract::{State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json};
use dill::Catalog;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utoipa::ToSchema;
use szfit_domain::entity::{Id, Workout, WorkoutWithOptionId};
use szfit_domain::services::IWorkoutListService;
use crate::app_response::{JsonResult};


#[derive(Serialize, Debug, ToSchema)]
#[serde(rename = "Workout")]
pub(super) struct WorkoutDto {
    id: Option<Id>,
    name: String,
}

impl From<Workout> for WorkoutDto {
    fn from(value: Workout) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
        }
    }
}

impl From<WorkoutWithOptionId> for WorkoutDto {
    fn from(value: WorkoutWithOptionId) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[utoipa::path(
    get,
    path = "/workout",
    responses(
        (status = 200, description = "Getting workout list")
    )
)]
pub(super) async fn workout_list(State(catalog): State<Catalog>,
                                 auth_claims: ExctractAuthClaims)
                                 -> JsonResult<Vec<WorkoutDto>> {
    let workout_service = catalog.get_one::<dyn IWorkoutListService>().unwrap();

    let workouts: Vec<WorkoutDto> = workout_service
        .list_workout_for_user(auth_claims.user_id)
        .await?
        .into_iter()
        .map(|w| w.into())
        .collect();

    Ok((StatusCode::OK, Json(workouts)))
}

#[derive(Deserialize)]
pub(super) struct WorkoutForCreate {
    name: String,
}

#[utoipa::path(
    post,
    path = "/workout",
    responses(
        (status = 201, description = "Create a workout")
    )
)]
pub(super) async fn create_workout(State(catalog): State<Catalog>,
                                   auth_claims: ExctractAuthClaims,
                                   Json(workout_for_create): Json<WorkoutForCreate>)
                                   -> impl IntoResponse {
    let workout_service = catalog.get_one::<dyn IWorkoutListService>().unwrap();

    let workout: WorkoutDto = workout_service
        .create_workout(auth_claims.user_id, workout_for_create.name)
        .await.unwrap()
        .into();
    (StatusCode::CREATED, Json(workout))
}

#[derive(Deserialize)]
pub(super) struct WorkoutForDelete {
    id: Id,
}

#[utoipa::path(
    delete,
    path = "/workout",
    responses(
        (status = 200, description = "Delete a workout")
    )
)]
pub(super) async fn delete_workout(State(catalog): State<Catalog>,
                                   _auth_claims: ExctractAuthClaims,
                                   Json(workout_for_delete): Json<WorkoutForDelete>)
                                   -> JsonResult<Value> {
    let workout_service = catalog.get_one::<dyn IWorkoutListService>().unwrap();

    workout_service
        .delete_workout(workout_for_delete.id)
        .await?;
    Ok((StatusCode::OK, Json(json!({
            "status": "ok",
        }))))
}

#[derive(Deserialize, ToSchema)]
pub(super) struct WorkoutForUpdate {
    id: Id,
    name: String,
}

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
pub(super) async fn update_workout(State(catalog): State<Catalog>,
                                   _: ExctractAuthClaims,
                                   Json(workout_for_update): Json<WorkoutForUpdate>)
                                   -> JsonResult<WorkoutDto> {
    let workout_service = catalog.get_one::<dyn IWorkoutListService>().unwrap();

    let workout: WorkoutDto = workout_service
        .update_workout(workout_for_update.id, workout_for_update.name)
        .await?
        .into();
    Ok((StatusCode::OK, Json(workout)))
}