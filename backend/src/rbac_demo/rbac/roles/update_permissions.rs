use crate::app_states::AppState;
use crate::errors::AppError;
use anyhow::Context;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::instrument;

#[instrument(
    name = "Add permissions to role",
    skip(app_state),
    fields(role_id = %role_id, permissions = ?permissions),
)]
pub async fn add_role_permissions(
    Path(role_id): Path<uuid::Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(permissions): Json<Vec<uuid::Uuid>>,
) -> Result<StatusCode, AppError> {
    let exists = check_role_exists(&app_state.pool, role_id)
        .await
        .map_err(AppError::E500)?;
    if !exists {
        return Ok(StatusCode::NOT_FOUND);
    }

    if !validate_permissions(&app_state.pool, &permissions)
        .await
        .map_err(AppError::E500)?
    {
        return Ok(StatusCode::NOT_FOUND);
    }

    let mut qb = sqlx::QueryBuilder::new("INSERT INTO roles_permissions (role_id, permission_id) ");
    qb.push_values(permissions, |mut query, permission| {
        query.push_bind(role_id);
        query.push_bind(permission);
    });
    qb.push(" ON CONFLICT (role_id, permission_id) DO NOTHING");
    qb.build()
        .execute(&app_state.pool)
        .await
        .context("Failed to insert new role into db")
        .map_err(AppError::E500)?;

    Ok(StatusCode::OK)
}

#[instrument(name = "Validate permissions", skip_all)]
async fn validate_permissions(
    pool: &PgPool,
    permissions: &[uuid::Uuid],
) -> Result<bool, anyhow::Error> {
    let all_exists = sqlx::query_scalar!(
        r#"
        SELECT NOT EXISTS (
            SELECT unnest($1::uuid[])
            EXCEPT
            SELECT permission_id FROM permissions
        ) as "all_exists!"
        "#,
        permissions as &[uuid::Uuid]
    )
    .fetch_one(pool)
    .await
    .context("Failed to check if permissions exist")?;

    Ok(all_exists)
}

#[instrument(
    name = "Remove permissions from role",
    skip(app_state),
    fields(role_id = %role_id, permissions = ?permissions),
)]
pub async fn remove_role_permissions(
    Path(role_id): Path<uuid::Uuid>,
    State(app_state): State<Arc<AppState>>,
    Json(permissions): Json<Vec<uuid::Uuid>>,
) -> Result<StatusCode, AppError> {
    let exists = check_role_exists(&app_state.pool, role_id)
        .await
        .map_err(AppError::E500)?;
    if !exists {
        return Ok(StatusCode::NOT_FOUND);
    }

    // No need to validate permissions since we are deleting them.
    // If a permission does not exist, it will simply be skipped.

    let result = sqlx::query!(
        r#"
        DELETE FROM roles_permissions 
        WHERE role_id = $1 AND permission_id = ANY($2)
        "#,
        role_id,
        &permissions as &[uuid::Uuid]
    )
    .execute(&app_state.pool)
    .await
    .context("Failed to delete permissions from role")
    .map_err(AppError::E500)?;

    tracing::info!("Deleted {} permissions from role", result.rows_affected());

    Ok(StatusCode::OK)
}

#[instrument(skip_all)]
async fn check_role_exists(pool: &PgPool, role_id: uuid::Uuid) -> Result<bool, anyhow::Error> {
    let exists = sqlx::query_scalar!("SELECT 1 FROM roles WHERE role_id = $1", role_id)
        .fetch_optional(pool)
        .await
        .context("Failed to check if role exists")?;

    Ok(exists.is_some())
}
