use crate::app_states::AppState;
use crate::errors::AppError;
use crate::rbac_demo::members::models::{CreateMember, Member};
use anyhow::Context;
use axum::extract::{Json, State};
use std::sync::Arc;
use tracing::instrument;

#[instrument(
    name = "Create a new member",
    skip(app_state),
    fields(
        first_name = request.first_name,
        last_name = request.last_name
    )
)]
pub async fn create_new_member(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreateMember>,
) -> Result<Json<Member>, AppError> {
    let member = sqlx::query_as!(
        Member,
        r#"
        INSERT INTO members (member_id, first_name, last_name)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING member_id, first_name, last_name
        "#,
        request.first_name,
        request.last_name,
    )
    .fetch_one(&app_state.pool)
    .await
    .context("Failed to create new member")
    .map_err(AppError::E500)?;

    Ok(Json(member))
}
