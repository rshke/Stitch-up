pub mod db {
    use crate::models::Filter;
    use anyhow::Context;
    use serde::Serialize;
    use sqlx::PgPool;
    use sqlx::QueryBuilder;

    pub async fn count<T>(table: &str, filter: T, pool: &PgPool) -> Result<i64, anyhow::Error>
    where
        T: Serialize,
    {
        let mut qb = QueryBuilder::new("SELECT count(*) FROM ");
        qb.push(table);

        Filter::to_query(&mut qb, &filter);

        qb.build_query_scalar()
            .fetch_one(pool)
            .await
            .context(format!("Failed to fetch count for table {}", table))
    }
}
