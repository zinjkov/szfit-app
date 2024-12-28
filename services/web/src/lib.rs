use crate::auth::{telegram_auth};
use crate::workout_list::ApiDocWorkout;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use szfit_domain::configure_catalog;
use szfit_domain::store::Db;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod auth;
mod workout_list;
mod app_response;
mod exercise;
mod workout_edit;

#[derive(OpenApi)]
#[openapi(paths(hello))]
pub struct ApiDoc;
pub async fn serve(db: Db) {
    let mut catalog_builder = configure_catalog();
    catalog_builder.add_value(db.clone());

    // Compose the routes
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .urls(
                vec![
                    (Url::with_primary("api doc 1", "/api-doc/openapi.json", true), ApiDoc::openapi()),
                    (Url::new("api doc 2", "/api-doc/openapi2.json"), ApiDocWorkout::openapi())
                ]))
        .route("/", get(hello))
        .route("/telegram_auth", post(telegram_auth))
        .merge(workout_list::router())
        .merge(exercise::router())
        .merge(workout_edit::router())
        .with_state(catalog_builder.build())
        .layer(TraceLayer::new_for_http());


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await
        .unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Send a salute from Axum")
    )
)]
pub async fn hello() -> impl IntoResponse {
    log::info!("telegram_auth hello world!");
    (StatusCode::OK, Json("hello world!"))
}


