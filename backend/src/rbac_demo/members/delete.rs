use crate::app_states::AppState;
use crate::errors::AppError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "Delete a member", skip(app_state))]
pub async fn delete_member(
    State(app_state): State<Arc<AppState>>,
    Path(member_id): Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    let deleted = delete_member_from_db(&app_state.pool, member_id)
        .await
        .map_err(AppError::E500)?;

    if !deleted {
        return Ok(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(name = "Try to delete the member from DB", skip_all)]
async fn delete_member_from_db(
    pool: &sqlx::PgPool,
    member_id: uuid::Uuid,
) -> Result<bool, anyhow::Error> {
    let count = sqlx::query!(
        r#"
        DELETE FROM members
        WHERE member_id = $1
        "#,
        member_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(count > 0)
}
