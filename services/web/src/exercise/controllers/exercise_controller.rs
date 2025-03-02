use crate::{
    app_response::WebResult,
    exercise::exercise_dto::{ExerciseDto, ExerciseForCreate},
};
use std::sync::Arc;
use szfit_domain::repositories::IExerciseRepository;
use szfit_macro::ControllerFromState;

#[derive(ControllerFromState)]
pub struct ExerciseController {
    exercise_repo: Arc<dyn IExerciseRepository>,
}

impl ExerciseController {
    // Возвращает список упражнений с ограничением по количеству
    pub async fn list(&self, limit: usize) -> WebResult<Vec<ExerciseDto>> {
        let limit = limit.min(100);
        Ok(self
            .exercise_repo
            .list_with_limit(limit)
            .await?
            .into_iter()
            .map(ExerciseDto::from)
            .collect())
    }

    // Создает новое упражнение
    pub async fn create(
        &self, exercise_for_create: ExerciseForCreate,
    ) -> WebResult<ExerciseDto> {
        Ok(self
            .exercise_repo
            .create(exercise_for_create.into())
            .await?
            .into())
    }
}
