use crate::app_states::AppState;
use crate::errors::AppError;
use crate::models::{ListRequest, ListResponse, Pagination};
use crate::rbac_demo::rbac::permissions::models::Permission;
use crate::rbac_demo::rbac::roles::models::Role;
use crate::utils::db;
use anyhow::Context;
use axum::extract::{Json, Path, State};
use serde_qs::axum::QsQuery;
use sqlx::QueryBuilder;
use std::sync::Arc;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn list_roles(
    QsQuery(request): QsQuery<ListRequest<()>>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<ListResponse<Role>>, AppError> {
    let mut qb = QueryBuilder::new("SELECT * FROM roles");
    Pagination::to_query(request.current_page, request.page_size, &mut qb);

    let roles = qb
        .build_query_as::<Role>()
        .fetch_all(&app_state.pool)
        .await
        .context("Failed to fetch roles")
        .map_err(AppError::E500)?;

    let total = db::count("roles", &(), &app_state.pool)
        .await
        .context("Failed to fetch roles count")
        .map_err(AppError::E500)?;

    Ok(Json(ListResponse {
        results: roles,
        total: total as u64,
        page: request.current_page,
    }))
}

#[instrument(skip_all)]
pub async fn list_role_permissions(
    Path(role_id): Path<uuid::Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Permission>>, AppError> {
    let roles = sqlx::query_as!(
        Permission,
        r#"SELECT p.*
        FROM permissions as p
        LEFT JOIN roles_permissions as rp ON p.permission_id = rp.permission_id
        WHERE rp.role_id = $1"#,
        role_id
    )
    .fetch_all(&app_state.pool)
    .await
    .context("Failed to fetch role")
    .map_err(AppError::E500)?;

    Ok(Json(roles))
}
