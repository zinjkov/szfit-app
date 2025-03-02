use serde::{Deserialize, Serialize};
use szfit_domain::entity::{Exercise, Id};
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct ExerciseDto {
    id: Option<Id>,
    name: String,
}

#[derive(Deserialize, Clone)]
pub(super) struct ExerciseForCreate {
    name: String,
}

impl From<Exercise> for ExerciseDto {
    fn from(value: Exercise) -> Self {
        Self { id: Some(value.id), name: value.name }
    }
}

impl Into<szfit_domain::repositories::ExerciseForCreate> for ExerciseForCreate {
    fn into(self) -> szfit_domain::repositories::ExerciseForCreate {
        szfit_domain::repositories::ExerciseForCreate { name: self.name }
    }
}
