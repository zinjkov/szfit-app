use serde::{Deserialize, Serialize};
use szfit_domain::entity::{Id, Workout, WorkoutWithOptionId};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename = "Workout")]
pub struct WorkoutDto {
    pub id: Option<Id>,
    pub name: String,
}

impl From<Workout> for WorkoutDto {
    fn from(value: Workout) -> Self {
        Self { id: Some(value.id), name: value.name }
    }
}

impl From<WorkoutWithOptionId> for WorkoutDto {
    fn from(value: WorkoutWithOptionId) -> Self {
        Self { id: value.id, name: value.name }
    }
}

#[derive(Deserialize)]
pub struct WorkoutForCreate {
    pub name: String,
}

#[derive(Deserialize)]
pub struct WorkoutForDelete {
    pub id: Id,
}

#[derive(Deserialize, ToSchema)]
pub struct WorkoutForUpdate {
    pub id: Id,
    pub name: String,
}
