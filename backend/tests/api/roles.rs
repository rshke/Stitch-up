use crate::helper::{insert_permissions, spawn_app};
use axum::http::StatusCode;
use backend::models::ListResponse;
use backend::rbac_demo::rbac::permissions::models::Permission;
use backend::rbac_demo::rbac::roles::models::Role;
use fake::Fake;
use serde_json::json;

#[tokio::test]
async fn roles_return_200_for_valid_data() {
    let app = spawn_app().await;
    let response = app
        .api_client
        .post(format!("{}/rbac-demo/roles", &app.address))
        .json(&json!({
            "name": "test",
            "description": "test",
        }))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");
    let role: Role = serde_json::from_str(&body).expect("Failed to parse response body");
    assert_eq!(role.name, "test");
}

#[tokio::test]
async fn persist_the_new_roles() {
    let app = spawn_app().await;
    let response = app
        .api_client
        .post(format!("{}/rbac-demo/roles", &app.address))
        .json(&json!({
            "name": "name",
            "description": "description",
        }))
        .send()
        .await
        .expect("Failed to post request");

    let saved = sqlx::query!(
        r#"
        SELECT * 
        FROM roles
        "#
    )
    .fetch_one(&app.pool)
    .await
    .unwrap();

    assert_eq!("name", saved.name);
    assert_eq!("description", saved.description);
}

#[tokio::test]
async fn add_permissions_to_role_success() {
    let app = spawn_app().await;
    let permissions = extract_permission_ids(insert_permissions(&app.pool, 2).await);
    let role = {
        let mut roles = insert_roles(&app.pool, 2).await;
        roles.pop().unwrap()
    };
    let response = app
        .api_client
        .post(format!(
            "{}/rbac-demo/roles/{}/permissions/add",
            &app.address, role.role_id
        ))
        .json(&json!(permissions))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), 200, "{}", response.text().await.unwrap());
}

#[tokio::test]
async fn remove_permissions_from_role_success() {
    let app = spawn_app().await;
    let permissions = extract_permission_ids(insert_permissions(&app.pool, 2).await);
    let role = {
        let mut roles = insert_roles(&app.pool, 2).await;
        roles.pop().unwrap()
    };

    app.api_client
        .post(format!(
            "{}/rbac-demo/roles/{}/permissions/add",
            &app.address, role.role_id
        ))
        .json(&json!(permissions))
        .send()
        .await
        .expect("Failed to post request");

    let response = app
        .api_client
        .post(format!(
            "{}/rbac-demo/roles/{}/permissions/remove",
            &app.address, role.role_id
        ))
        .json(&json!(permissions))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), 200, "{}", response.text().await.unwrap());
}

#[tokio::test]
async fn invalid_role_will_should_rejected() {
    let app = spawn_app().await;
    let permissions = extract_permission_ids(insert_permissions(&app.pool, 2).await);
    let role = {
        let mut roles = insert_roles(&app.pool, 2).await;
        roles.pop().unwrap()
    };

    let response = app
        .api_client
        .post(format!(
            "{}/rbac-demo/roles/{}/permissions/add",
            &app.address,
            uuid::Uuid::new_v4()
        ))
        .json(&json!(permissions))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn invalid_permissions_should_be_rejected() {
    let app = spawn_app().await;
    let permissions = extract_permission_ids(insert_permissions(&app.pool, 2).await);
    let role = {
        let mut roles = insert_roles(&app.pool, 2).await;
        roles.pop().unwrap()
    };

    let response = app
        .api_client
        .post(format!(
            "{}/rbac-demo/roles/{}/permissions/add",
            &app.address, role.role_id
        ))
        .json(&json!([vec![uuid::Uuid::new_v4()], permissions].concat()))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_roles_success() {
    let app = spawn_app().await;
    let roles = insert_roles(&app.pool, 2).await;
    let response = app
        .api_client
        .get(format!("{}/rbac-demo/roles", &app.address))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");
    let response_body: ListResponse<Role> =
        serde_json::from_str(&body).expect("Failed to parse response body");
    assert_eq!(response_body.total, roles.len() as u64);
    assert_eq!(response_body.page, 1);
    assert_eq!(response_body.results.len(), roles.len());
}

#[tokio::test]
async fn list_role_permissions_success() {
    let app = spawn_app().await;
    let role = {
        let mut roles = insert_roles(&app.pool, 2).await;
        roles.pop().unwrap()
    };
    let permissions = extract_permission_ids(insert_permissions(&app.pool, 2).await);
    app.api_client
        .post(format!(
            "{}/rbac-demo/roles/{}/permissions/add",
            &app.address, role.role_id
        ))
        .json(&json!(permissions))
        .send()
        .await
        .expect("Failed to post request");

    let response = app
        .api_client
        .get(format!(
            "{}/rbac-demo/roles/{}/permissions",
            &app.address, role.role_id
        ))
        .send()
        .await
        .expect("Failed to post request");

    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to read response body");
    let permissions: Vec<Permission> =
        serde_json::from_str(&body).expect("Failed to parse response body");
    assert_eq!(permissions.len(), 2);
}

async fn insert_roles(pgpool: &sqlx::PgPool, amount: u64) -> Vec<Role> {
    let roles: Vec<Role> = (1..=amount)
        .map(|_| Role {
            role_id: uuid::Uuid::new_v4(),
            name: fake::faker::lorem::en::Word().fake::<String>(),
            description: fake::faker::lorem::en::Sentence(1..5).fake::<String>(),
        })
        .collect::<Vec<_>>();

    let mut query_builder =
        sqlx::QueryBuilder::new("INSERT INTO roles (role_id, name, description) ");
    query_builder.push_values(roles.clone(), |mut query, role| {
        query
            .push_bind(role.role_id)
            .push_bind(role.name)
            .push_bind(role.description);
    });
    let query = query_builder.build();
    query.execute(pgpool).await.unwrap();

    roles
}

fn extract_permission_ids(permissions: Vec<Permission>) -> Vec<uuid::Uuid> {
    permissions.into_iter().map(|p| p.permission_id).collect()
}
