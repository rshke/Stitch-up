mod admin;
mod health_check;
pub mod session_state;
mod user;

use std::sync::Arc;

use axum::extract::Request;
use axum::middleware::{Next, from_fn};
use axum::response::Response;
use axum::routing::{get, post};
use axum_session::{SessionLayer, SessionStore};
use axum_session_redispool::SessionRedisPool;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::app_states::AppState;
use crate::rbac_demo;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub fn get_router(
    pool: Pool<Postgres>,
    base_url: String,
    session_store: SessionStore<SessionRedisPool>,
) -> axum::Router {
    let app_state = Arc::new(AppState { pool, base_url });

    // TODO: Restrict the origin to the frontend URL
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    axum::Router::new()
        .route("/health", get(health_check::health_check))
        .route("/login", post(user::login))
        .nest("/rbac-demo", rbac_demo::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(SessionLayer::new(session_store))
        // .layer(from_fn(reject_anonymous_users))
        .layer(from_fn(log_app_errors))
        .with_state(app_state)
}

async fn log_app_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    if let Some(err) = response.extensions().get::<Arc<anyhow::Error>>() {
        tracing::error!(?err, "an unexpected error occurred inside a handler");
    }
    response
}
