use crate::services::training_service::implementation::training_service::TrainingService;
use crate::services::ITrainingService;
use dill::CatalogBuilder;

mod implementation;
pub mod training_service;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder
        .add::<TrainingService>()
        .bind::<dyn ITrainingService, TrainingService>();
}
