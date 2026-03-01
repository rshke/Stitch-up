use sqlx::FromRow;

#[derive(Debug, serde::Deserialize, serde::Serialize, FromRow, Clone)]
pub struct Permission {
    pub permission_id: uuid::Uuid,
    pub resource: String,
    pub action: String,
    pub scope: String,
}
