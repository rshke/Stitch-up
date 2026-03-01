use crate::helper::{insert_permissions, spawn_app};
use backend::models::{ListRequest, ListResponse};
use backend::rbac_demo::rbac::permissions::get::PermissionFilter;
use backend::rbac_demo::rbac::permissions::models::Permission;
use std::collections::HashSet;

#[tokio::test]
async fn return_valid_permissions_list() {
    let app = spawn_app().await;

    let amount = 9;
    let resources = extract_resources(insert_permissions(&app.pool, amount).await);

    let reponse = app
        .api_client
        .get(&format!("{}/rbac-demo/permissions", &app.address))
        .query(&[("current_page", "1"), ("page_size", "10")])
        .send()
        .await
        .expect("Failed to send request");
    assert!(reponse.status().is_success());

    let response_body = reponse.json::<ListResponse<Permission>>().await.unwrap();
    assert_eq!(response_body.total, amount);
    assert_eq!(response_body.page, 1);
    assert_eq!(response_body.results.len(), amount as usize);

    for permission in response_body.results {
        assert!(resources.contains(&permission.resource));
    }
}

#[tokio::test]
async fn return_valid_permissions_list_with_filter() {
    let app = spawn_app().await;

    let amount = 9;
    let resources = extract_resources(insert_permissions(&app.pool, amount).await);

    let request = ListRequest::<PermissionFilter> {
        current_page: 1,
        page_size: 10,
        filter: Some(PermissionFilter {
            resource: Some(resources.iter().next().unwrap().clone()),
        }),
    };

    let reponse = app
        .api_client
        .get(&format!(
            "{}/rbac-demo/permissions?{}",
            &app.address,
            serde_qs::to_string(&request).unwrap()
        ))
        .send()
        .await
        .expect("Failed to send request");
    assert!(reponse.status().is_success());

    let response_body = reponse.json::<ListResponse<Permission>>().await.unwrap();
    assert_eq!(response_body.total, 1);
    assert_eq!(response_body.page, 1);
    assert_eq!(response_body.results.len(), 1);

    for permission in response_body.results {
        assert!(resources.contains(&permission.resource));
    }
}

fn extract_resources(permissions: Vec<Permission>) -> HashSet<String> {
    permissions
        .into_iter()
        .map(|p| p.resource)
        .collect::<HashSet<String>>()
}
