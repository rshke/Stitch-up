use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow, Clone)]
pub struct Role {
    pub role_id: uuid::Uuid,
    pub name: String,
    pub description: String,
}

impl Role {
    pub fn new(role_id: uuid::Uuid, name: String, description: String) -> Self {
        Self {
            role_id,
            name,
            description,
        }
    }

    pub async fn add_permissions(
        &self,
        pgpool: &sqlx::PgPool,
        permissions: impl AsRef<[uuid::Uuid]>,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder =
            sqlx::QueryBuilder::new("INSERT INTO role_permissions (role_id, permission_id) ");
        query_builder.push_values(permissions.as_ref(), |mut query, permission| {
            query.push_bind(self.role_id).push_bind(permission);
        });
        query_builder.push(" ON CONFLICT (role_id, permission_id) DO NOTHING");
        query_builder.build().execute(pgpool).await?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateRole {
    pub name: String,
    pub description: String,
}
