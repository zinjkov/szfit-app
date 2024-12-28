extern crate core;

use crate::repositories::workout_plan_repo;
use dill::CatalogBuilder;

pub mod aggregate;
pub mod entity;
pub mod repositories;
pub mod services;
pub mod store;
pub mod usecases;
mod jwt_auth;



pub fn configure_catalog() -> CatalogBuilder {
    let mut b = CatalogBuilder::new();

    // Workout Plan
    services::workout_plan_service::register_in_catalog(&mut b);
    workout_plan_repo::register_in_catalog(&mut b);

    // Auth
    repositories::user_repo::register_in_catalog(&mut b);
    services::auth_service::register_in_catalog(&mut b);

    //Workout
    services::workout_list_service::register_in_catalog(&mut b);

    // Exercise
    repositories::exercise_repo::register_in_catalog(&mut b);

    // Sets
    repositories::sets_repo::register_in_catalog(&mut b);

    // JWT
    services::jwt_service::register_in_catalog(&mut b);

    // Training
    services::training_service::register_in_catalog(&mut b);
    repositories::training_repo::register_in_catalog(&mut b);

    b
}

