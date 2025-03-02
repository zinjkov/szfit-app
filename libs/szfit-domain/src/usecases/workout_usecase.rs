use std::collections::BTreeSet;
use std::sync::Arc;

use crate::aggregate::Sets;
use crate::entity::{Id, Training};
use crate::repositories::{ISetsRepository, SetsForCreate};
use crate::services::{ITrainingService, StartTrainingArgs};

pub struct WorkoutUsecase {
    training: Box<Training>,
    sets_repository: Arc<dyn ISetsRepository>,
    training_service: Arc<dyn ITrainingService>,
    exercise_in_progress: Option<Id>,
    has_sets: BTreeSet<Id>,
}

impl WorkoutUsecase {
    pub async fn new(
        user_id: Id,
        workout_plan_id: Id,
        sets_repository: Arc<dyn ISetsRepository>,
        training_service: Arc<dyn ITrainingService>,
    ) -> Option<Self> {
        let training = training_service
            .start_training(StartTrainingArgs::new(
                user_id,
                workout_plan_id,
                None,
            ))
            .await
            .ok()?;

        Some(Self {
            training: Box::new(training),
            training_service,
            sets_repository,
            exercise_in_progress: None,
            has_sets: BTreeSet::new(),
        })
    }

    pub fn current_workout(&self) -> Id {
        self.training.workout_plan_id
    }

    pub async fn push_sets(&mut self, sets: Sets) {
        if let Some(exercise_id) = &self.exercise_in_progress {
            log::info!("Pushed exercise {:?}", exercise_id);
            let _ = self
                .sets_repository
                .create(vec![SetsForCreate {
                    weight_kg: sets.weight_kg,
                    reps: sets.reps,
                    user_id: self.training.user_id,
                    exercise_id: *exercise_id,
                    training_id: self.training.workout_plan_id,
                }])
                .await;

            self.has_sets.insert(*exercise_id);
        }
    }

    pub async fn last_max_set(&mut self) -> Option<crate::entity::Sets> {
        self.sets_repository
            .last_max_set(
                self.training.user_id,
                self.exercise_in_progress?,
            )
            .await
            .ok()
    }

    pub fn start_exercise(&mut self, exercise_id: Id) {
        self.exercise_in_progress = Some(exercise_id);
    }

    pub async fn finish_workout(&self) -> Option<()> {
        log::info!(
            "Workout {:?} finished for user {:?}",
            self.training.workout_plan_id,
            self.training.user_id
        );
        self.training_service
            .finish_training(self.training.id)
            .await
            .ok()?;
        Some(())
    }

    pub fn has_sets(&self, id: &Id) -> bool {
        self.has_sets.contains(id)
    }
}

#[cfg(test)]
mod tests {}
