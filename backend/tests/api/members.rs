use std::collections::HashMap;

use backend::models::ListResponse;
use backend::rbac_demo::members::models::Member;
use fake::{Fake, faker};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::helper::spawn_app;

#[tokio::test]
async fn return_200_for_valid_member_data() {
    let app = spawn_app().await;

    let data = HashMap::from([
        (
            "first_name",
            faker::name::zh_cn::FirstName().fake::<String>(),
        ),
        ("last_name", faker::name::zh_cn::LastName().fake::<String>()),
    ]);
    let response = app
        .api_client
        .post(format!("{}/rbac-demo/members", app.address))
        .json(&json!(data))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn persist_the_new_member() {
    let app = spawn_app().await;

    let data = HashMap::from([
        (
            "first_name",
            faker::name::zh_cn::FirstName().fake::<String>(),
        ),
        ("last_name", faker::name::zh_cn::LastName().fake::<String>()),
    ]);
    app.api_client
        .post(format!("{}/rbac-demo/members", app.address))
        .json(&json!(data))
        .send()
        .await
        .unwrap();

    let record = sqlx::query!("SELECT first_name, last_name FROM members")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch records");

    assert_eq!(record.first_name, data["first_name"]);
    assert_eq!(record.last_name, data["last_name"]);
}

#[tokio::test]
async fn return_valid_members_list() {
    let app = spawn_app().await;

    let amount = 9;
    insert_members(&app.pool, amount).await;

    let response = app
        .api_client
        .get(&format!("{}/rbac-demo/members", app.address))
        .query(&[("current_page", "1"), ("page_size", "10")])
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());

    let response_body = response.json::<ListResponse<Member>>().await.unwrap();
    assert_eq!(response_body.results.len(), amount as usize);
    assert_eq!(response_body.total, amount);
    assert_eq!(response_body.page, 1);
}

#[tokio::test]
async fn return_204_if_successfully_deleted() {
    let app = spawn_app().await;

    let members = insert_members(&app.pool, 1).await;
    let member_id = members.first().unwrap().member_id;

    let response = app
        .api_client
        .delete(&format!("{}/rbac-demo/members/{}", app.address, member_id))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn return_404_if_member_not_found() {
    let app = spawn_app().await;

    let response = app
        .api_client
        .delete(&format!(
            "{}/rbac-demo/members/{}",
            app.address,
            uuid::Uuid::new_v4()
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

async fn insert_members(pool: &PgPool, amount: u64) -> Vec<Member> {
    let members = (0..amount)
        .map(|_| Member {
            member_id: uuid::Uuid::new_v4(),
            first_name: faker::name::zh_cn::FirstName().fake::<String>(),
            last_name: faker::name::zh_cn::LastName().fake::<String>(),
        })
        .collect::<Vec<Member>>();

    let mut qb = sqlx::QueryBuilder::new("INSERT INTO members (member_id, first_name, last_name) ");
    qb.push_values(members.clone(), |mut b, member| {
        b.push_bind(member.member_id)
            .push_bind(member.first_name)
            .push_bind(member.last_name);
    });
    qb.build().execute(pool).await.unwrap();

    members
}
