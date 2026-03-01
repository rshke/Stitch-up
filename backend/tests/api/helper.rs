use argon2::Argon2;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use backend::configuration::{DBSettings, Settings};
use backend::rbac_demo::rbac::permissions::models::Permission;
use backend::startup::Application;
use backend::telemetry::{get_subscriber, init_subscriber};
use fake::Fake;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name;
use once_cell::sync::Lazy;
use rand::distr::{Distribution, slice::Choose};
use rand::rng;
use reqwest::Response;
use reqwest::Url;
use reqwest::redirect::Policy;
use serde_json::{Value, json};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    fn generate() -> Self {
        TestUser {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        sqlx::query!(
            r#"
            INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)
            "#,
            self.user_id,
            self.username,
            password_hash
        )
        .execute(pool)
        .await
        .expect("Failed to create test users.");
    }
}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub pool: PgPool,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
}

pub struct ConfirmationLinks {
    pub html: Url,
    pub pain_text: Url,
}

impl TestApp {
    pub async fn post_login(&self, body: &Value) -> reqwest::Response {
        self.api_client
            .post(format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn login(&self) {
        self.post_login(&json!({
            "username": self.test_user.username,
            "password": self.test_user.password,
        }))
        .await;
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.api_client
            .post(format!("{}/admin/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_change_password<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/admin/password", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to post request")
    }
}

static INIT_SUBSCRIBER: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&INIT_SUBSCRIBER);

    let app_config = get_test_config();

    let pool = configure_database(&app_config.database).await;

    let app = Application::build(app_config)
        .await
        .expect("Failed to build application");

    let app_port = app.port();
    let app_url = format!("http://127.0.0.1:{}", app_port);

    tokio::spawn(app.run_until_stop());

    let test_user = TestUser::generate();
    test_user.store(&pool).await;

    let api_client = reqwest::Client::builder()
        .cookie_store(true)
        // Do not follow redirects automatically
        .redirect(Policy::none())
        // Do not use proxy
        .no_proxy()
        .build()
        .unwrap();

    TestApp {
        address: app_url,
        port: app_port,
        pool,
        test_user,
        api_client,
    }
}

fn get_test_config() -> Settings {
    let mut c = backend::configuration::get_config().expect("Failed to load configuration");
    c.app_settings.port = 0;

    c.database.database_name = format!(
        "test_{}",
        uuid::Uuid::new_v4().to_string().replace('-', "_")
    );

    c
}

async fn configure_database(configuration: &DBSettings) -> PgPool {
    let url = configuration.get_connection_without_database();
    let mut db_connection = PgConnection::connect(&url)
        .await
        .unwrap_or_else(|_| panic!("Failed to connect to postgres server: {}", url));
    db_connection
        .execute(format!("CREATE DATABASE {};", configuration.database_name).as_str())
        .await
        .expect("Failed to create database");
    db_connection
        .close()
        .await
        .expect("Failed to close connection");

    let pool = sqlx::PgPool::connect(configuration.get_connection().as_str())
        .await
        .expect("Failed to connect to the database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub fn valid_subscriber() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let name: String = name::en::Name().fake();
    let email: String = SafeEmail().fake();
    map.insert("name".to_string(), name);
    map.insert("email".to_string(), email);

    map
}

pub fn assert_is_redirect_to(response: &reqwest::Response, url: &str) {
    let status = response.status();
    assert!(
        status.is_redirection(),
        "Expected redirect status, got {}",
        status
    );

    let location = response
        .headers()
        .get(reqwest::header::LOCATION)
        .expect("Missing Location header")
        .to_str()
        .expect("Invalid Location header value");

    assert_eq!(
        location, url,
        "Expected redirect to `{}`, but got `{}`",
        url, location
    );
}

pub async fn insert_permissions(pool: &sqlx::PgPool, amount: u64) -> Vec<Permission> {
    let actions = ["read", "write", "delete", "update"];
    let actions_dist = Choose::new(&actions).unwrap();
    let scopes = ["*", "self", "team"];
    let scopes_dist = Choose::new(&scopes).unwrap();
    let mut rng = rng();

    let permissions: Vec<Permission> = (1..=amount)
        .map(|_| Permission {
            permission_id: uuid::Uuid::new_v4(),
            resource: fake::faker::lorem::en::Word().fake::<String>(),
            action: (*actions_dist.sample(&mut rng)).to_string(),
            scope: (*scopes_dist.sample(&mut rng)).to_string(),
        })
        .collect::<Vec<_>>();

    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO permissions (permission_id, resource, action, scope) ",
    );
    query_builder.push_values(permissions.clone(), |mut query, p| {
        query
            .push_bind(p.permission_id)
            .push_bind(p.resource)
            .push_bind(p.action)
            .push_bind(p.scope);
    });
    let query = query_builder.build();
    query.execute(pool).await.unwrap();

    permissions
}
