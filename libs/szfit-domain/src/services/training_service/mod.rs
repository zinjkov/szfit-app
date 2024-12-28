use dill::CatalogBuilder;
use crate::services::ITrainingService;
use crate::services::training_service::implementation::training_service::TrainingService;

pub mod training_service;
mod implementation;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<TrainingService>()
        .bind::<dyn ITrainingService, TrainingService>();
}