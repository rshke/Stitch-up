use std::collections::HashMap;

use backend::models::ListResponse;
use backend::rbac_demo::projects::models::Project;
use fake::{Fake, faker};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::helper::spawn_app;

#[tokio::test]
async fn return_200_for_valid_project_data() {
    let app = spawn_app().await;

    let data = HashMap::from([
        ("name", fake::faker::lorem::en::Word().fake::<String>()),
        (
            "description",
            fake::faker::lorem::en::Sentence(3..5).fake::<String>(),
        ),
    ]);
    let response = app
        .api_client
        .post(format!("{}/rbac-demo/projects", app.address))
        .json(&json!(data))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn persist_the_new_project() {
    let app = spawn_app().await;

    let data = HashMap::from([
        ("name", faker::lorem::en::Word().fake::<String>()),
        (
            "description",
            fake::faker::lorem::en::Sentence(3..5).fake::<String>(),
        ),
    ]);
    app.api_client
        .post(format!("{}/rbac-demo/projects", app.address))
        .json(&json!(data))
        .send()
        .await
        .unwrap();

    let record = sqlx::query!("SELECT name, description FROM projects")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch records");

    assert_eq!(record.name, data["name"]);
    assert_eq!(record.description, data["description"]);
}

#[tokio::test]
async fn return_valid_projects_list() {
    let app = spawn_app().await;

    let amount = 9;
    insert_projects(&app.pool, amount).await;

    let response = app
        .api_client
        .get(&format!("{}/rbac-demo/projects", app.address))
        .query(&[("current_page", "1"), ("page_size", "10")])
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());

    let response_body = response.json::<ListResponse<Project>>().await.unwrap();
    assert_eq!(response_body.results.len(), amount as usize);
    assert_eq!(response_body.total, amount);
    assert_eq!(response_body.page, 1);
}

#[tokio::test]
async fn return_204_if_successfully_deleted() {
    let app = spawn_app().await;

    let projects = insert_projects(&app.pool, 1).await;
    let project_id = projects.first().unwrap().project_id;

    let response = app
        .api_client
        .delete(&format!(
            "{}/rbac-demo/projects/{}",
            app.address, project_id
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn return_404_if_project_not_found() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .delete(&format!(
            "{}/rbac-demo/projects/{}",
            app.address,
            uuid::Uuid::new_v4()
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

async fn insert_projects(pool: &PgPool, amount: u64) -> Vec<Project> {
    let projects = (0..amount)
        .map(|_| Project {
            project_id: uuid::Uuid::new_v4(),
            name: faker::lorem::en::Word().fake::<String>(),
            description: faker::lorem::en::Sentence(3..5).fake::<String>(),
        })
        .collect::<Vec<Project>>();

    let mut qb = sqlx::QueryBuilder::new("INSERT INTO projects (project_id, name, description) ");
    qb.push_values(projects.clone(), |mut b, project| {
        b.push_bind(project.project_id)
            .push_bind(project.name)
            .push_bind(project.description);
    });
    qb.build().execute(pool).await.unwrap();

    projects
}
