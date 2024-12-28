extern crate core;

use crate::jwt_auth::JwtSecret;
use crate::repositories::workout_plan_repo;
use crate::services::{AccessTokenExpTime, RefreshTokenExpTime};
use chrono::Duration;
use dill::CatalogBuilder;

pub mod aggregate;
pub mod entity;
pub mod repositories;
pub mod services;
pub mod store;
pub mod usecases;
mod jwt_auth;

macro_rules! add_to {
    ($x:expr, $t:ty) => {{
        $x.add::<$t>();
    }};
}

macro_rules! add_to_with_bind {
    ($x:expr, $t:ty, $d:path) => {{
        $x.add::<$t>()
        .bind::<dyn $d, $t>();
    }};
}


pub fn configure_catalog() -> CatalogBuilder {
    let mut b = CatalogBuilder::new();


    b.add_value(JwtSecret(String::from("secret")));
    b.add_value(AccessTokenExpTime(Duration::days(30)));
    b.add_value(RefreshTokenExpTime(Duration::days(120)));

    // Workout Plan

    services::workout_plan_service::register_in_catalog(&mut b);
    workout_plan_repo::register_in_catalog(&mut b);

    // Auth
    repositories::user_repo::register_in_catalog(&mut b);

    add_to!(b, services::AuthService);

    //Workout
    services::workout_list_service::register_in_catalog(&mut b);

    // Exercise
    repositories::exercise_repo::register_in_catalog(&mut b);

    // Sets
    repositories::sets_repo::register_in_catalog(&mut b);

    // Jwt
    add_to!(b, services::JwtService);

    // JwtAuthenticator
    add_to_with_bind!(b,
        services::jwt_authenticator::JwtAuthenticator,
        services::jwt_authenticator::IJwtAuthenticator);

    // Training
    add_to!(b, services::TrainingService);
    repositories::training_repo::register_in_catalog(&mut b);

    b
}

