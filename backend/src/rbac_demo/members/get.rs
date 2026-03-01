use crate::app_states::AppState;
use crate::errors::AppError;
use crate::models::{ListRequest, ListResponse, Pagination};
use crate::rbac_demo::members::models::Member;
use crate::utils::db;
use anyhow::Context;
use axum::extract::{Json, Query, State};
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "List all members", skip_all)]
pub async fn list_members(
    State(app_state): State<Arc<AppState>>,
    Query(request): Query<ListRequest<()>>,
) -> Result<Json<ListResponse<Member>>, AppError> {
    let mut qb = sqlx::QueryBuilder::new("SELECT member_id, first_name, last_name FROM members");

    Pagination::to_query(request.current_page, request.page_size, &mut qb);

    let members = qb
        .build_query_as::<Member>()
        .fetch_all(&app_state.pool)
        .await
        .context("Failed to fetch members")
        .map_err(AppError::E500)?;

    let total = db::count("members", &(), &app_state.pool)
        .await
        .context("Failed to fetch members count")
        .map_err(AppError::E500)?;

    Ok(Json(ListResponse {
        results: members,
        total: total as u64,
        page: request.current_page,
    }))
}
