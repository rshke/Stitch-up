use super::models::Role;
use crate::app_states::AppState;
use crate::{errors::AppError, rbac_demo::rbac::roles::models::CreateRole};
use anyhow::Context;
use axum::extract::Json;
use axum::extract::State;
use std::sync::Arc;

pub async fn create_new_role(
    State(app_state): State<Arc<AppState>>,
    Json(role): Json<CreateRole>,
) -> Result<Json<Role>, AppError> {
    let role = sqlx::query_as!(
        Role,
        r#"
        INSERT INTO roles (role_id, name, description)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING role_id, name, description
        "#,
        role.name,
        role.description
    )
    .fetch_one(&app_state.pool)
    .await
    .context("Failed to insert new role into db")
    .map_err(AppError::E500)?;

    Ok(Json(role))
}
