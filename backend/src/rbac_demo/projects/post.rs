use crate::app_states::AppState;
use crate::errors::AppError;
use crate::rbac_demo::projects::models::{CreateProject, Project};
use anyhow::Context;
use axum::extract::{Json, State};
use std::sync::Arc;
use tracing::instrument;

#[instrument(
    name = "Create a new project",
    skip(app_state, request),
    fields(name = request.name)
)]
pub async fn create_new_project(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreateProject>,
) -> Result<Json<Project>, AppError> {
    let project = sqlx::query_as!(
        Project,
        r#"
        INSERT INTO projects (project_id, name, description)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING project_id, name, description
        "#,
        request.name,
        request.description,
    )
    .fetch_one(&app_state.pool)
    .await
    .context("Failed to create new project")
    .map_err(AppError::E500)?;

    Ok(Json(project))
}
