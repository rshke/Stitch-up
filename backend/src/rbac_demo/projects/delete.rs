use crate::app_states::AppState;
use crate::errors::AppError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "Delete a project", skip(app_state))]
pub async fn delete_project(
    State(app_state): State<Arc<AppState>>,
    Path(project_id): Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    let deleted = delete_project_from_db(&app_state.pool, project_id)
        .await
        .map_err(AppError::E500)?;

    if !deleted {
        return Ok(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(name = "Try to delete the project from DB", skip_all)]
async fn delete_project_from_db(
    pool: &sqlx::PgPool,
    project_id: uuid::Uuid,
) -> Result<bool, anyhow::Error> {
    let count = sqlx::query!(
        r#"
        DELETE FROM projects
        WHERE project_id = $1
        "#,
        project_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(count > 0)
}
