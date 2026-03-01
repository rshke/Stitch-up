use crate::utils::db;
use anyhow::Context;
use axum::extract::{Json, Query, State};
use serde::{Deserialize, Serialize};
use serde_qs::axum::QsQuery;
use sqlx::QueryBuilder;
use std::sync::Arc;
use tracing::instrument;

use crate::app_states::AppState;
use crate::errors::AppError;
use crate::models::{Filter, ListRequest, ListResponse, Pagination};
use crate::rbac_demo::rbac::permissions::models::Permission;

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionFilter {
    pub resource: Option<String>,
}

#[axum::debug_handler]
#[instrument(name = "get_permissions", skip_all)]
pub async fn list_permissions(
    QsQuery(request): QsQuery<ListRequest<PermissionFilter>>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<ListResponse<Permission>>, AppError> {
    let mut qb = QueryBuilder::new(
        r#"
        SELECT permission_id, resource, action, scope
        FROM permissions
        "#,
    );
    if let Some(filter) = &request.filter {
        Filter::to_query(&mut qb, filter);
    }
    Pagination::to_query(request.current_page, request.page_size, &mut qb);

    let permissions = qb
        .build_query_as::<Permission>()
        .fetch_all(&app_state.pool)
        .await
        .context("Failed to fetch permissions from db")
        .map_err(AppError::E500)?;

    let total = db::count("permissions", request.filter, &app_state.pool)
        .await
        .context("Failed to fetch permissions from db")
        .map_err(AppError::E500)?;

    Ok(Json(ListResponse {
        results: permissions,
        total: total as u64,
        page: request.current_page,
    }))
}
