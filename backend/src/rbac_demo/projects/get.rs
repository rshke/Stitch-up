use crate::app_states::AppState;
use crate::errors::AppError;
use crate::models::{ListRequest, ListResponse, Pagination};
use crate::rbac_demo::projects::models::Project;
use crate::utils::db;
use anyhow::Context;
use axum::extract::{Json, Query, State};
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "List all projects", skip_all)]
pub async fn list_projects(
    State(app_state): State<Arc<AppState>>,
    Query(request): Query<ListRequest<()>>,
) -> Result<Json<ListResponse<Project>>, AppError> {
    let mut qb = sqlx::QueryBuilder::new(
        r#"
        SELECT project_id, name, description
        FROM projects
        "#,
    );

    Pagination::to_query(request.current_page, request.page_size, &mut qb);

    let projects = qb
        .build_query_as::<Project>()
        .fetch_all(&app_state.pool)
        .await
        .context("Failed to fetch projects")
        .map_err(AppError::E500)?;

    let total = db::count("projects", &(), &app_state.pool)
        .await
        .context("Failed to fetch projects count")
        .map_err(AppError::E500)?;

    Ok(Json(ListResponse {
        results: projects,
        total: total as u64,
        page: request.current_page,
    }))
}
