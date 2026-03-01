use crate::app_states::AppState;
use axum::routing::{delete, get, post};
use std::sync::Arc;
pub mod members;
pub mod projects;
pub mod rbac;

pub fn router() -> axum::routing::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/roles", post(rbac::roles::post::create_new_role))
        .route(
            "/roles/{id}/permissions/add",
            post(rbac::roles::update_permissions::add_role_permissions),
        )
        .route(
            "/roles/{id}/permissions/remove",
            post(rbac::roles::update_permissions::remove_role_permissions),
        )
        .route("/roles", get(rbac::roles::get::list_roles))
        .route(
            "/roles/{id}/permissions",
            get(rbac::roles::get::list_role_permissions),
        )
        .route(
            "/permissions",
            get(rbac::permissions::get::list_permissions),
        )
        .route("/members", post(members::post::create_new_member))
        .route("/members", get(members::get::list_members))
        .route("/members/{id}", delete(members::delete::delete_member))
        .route("/projects", post(projects::post::create_new_project))
        .route("/projects", get(projects::get::list_projects))
        .route("/projects/{id}", delete(projects::delete::delete_project))
}
