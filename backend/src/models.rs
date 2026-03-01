use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};
use sqlx::Postgres;
use sqlx::QueryBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub total: u64,
    pub page: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListRequest<T> {
    pub filter: Option<T>,
    #[serde(default = "default_page")]
    pub current_page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

pub struct Pagination;

impl Pagination {
    pub fn to_query(current_page: u64, page_size: u64, qb: &mut QueryBuilder<Postgres>) {
        let offset = (current_page - 1) * page_size;
        let limit = page_size;

        qb.push(" LIMIT ").push_bind(limit as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);
    }
}

pub struct Filter;

impl Filter {
    pub fn to_query<T>(qb: &mut QueryBuilder<Postgres>, filter: &T)
    where
        T: Serialize,
    {
        let conditions = to_value(filter).unwrap();
        if let Value::Object(conditions) = conditions {
            qb.push(" WHERE ");

            let mut current_condition = qb.separated(" AND ");
            current_condition.push("TRUE");
            for (key, value) in conditions.into_iter() {
                match value {
                    Value::Null => continue,
                    Value::Bool(v) => {
                        current_condition
                            .push(format!("{} = ", key))
                            .push_bind_unseparated(v);
                    }
                    Value::Number(v) => {
                        current_condition
                            .push(format!("{} = ", key))
                            .push_bind_unseparated(v.as_f64().unwrap());
                    }
                    Value::String(v) => {
                        current_condition
                            .push(format!("{} = ", key))
                            .push_bind_unseparated(v);
                    }
                    _ => {
                        tracing::warn!("Unsupported filter type: {}", value);
                        continue;
                    }
                }
            }
        }
    }
}

mod tests {

    use sqlx::Execute;

    use super::*;

    #[derive(Serialize)]
    struct UserFilter {
        name: Option<String>,
        age: Option<u64>,
    }

    #[test]
    fn test_pagination_to_query() {
        let mut qb = QueryBuilder::new("SELECT * FROM users");
        Pagination::to_query(1, 10, &mut qb);
        assert_eq!(qb.build().sql(), "SELECT * FROM users LIMIT $1 OFFSET $2");
    }

    #[test]
    fn test_filter_to_query() {
        let filter = UserFilter {
            name: Some("John".to_string()),
            age: None,
        };
        let mut qb = QueryBuilder::new("SELECT * FROM users");
        Filter::to_query(&mut qb, &filter);
        assert_eq!(
            qb.build().sql(),
            "SELECT * FROM users WHERE TRUE AND name = $1"
        );
    }

    #[test]
    fn test_filter_to_query_with_all_none() {
        let filter = UserFilter {
            name: None,
            age: None,
        };
        let mut qb = QueryBuilder::new("SELECT * FROM users");
        Filter::to_query(&mut qb, &filter);
        assert_eq!(qb.build().sql(), "SELECT * FROM users WHERE TRUE");
    }

    #[test]
    fn test_pagination_and_filter_to_query() {
        let mut qb = QueryBuilder::new("SELECT * FROM users");
        Filter::to_query(
            &mut qb,
            &UserFilter {
                name: Some("John".to_string()),
                age: None,
            },
        );
        Pagination::to_query(1, 10, &mut qb);

        assert_eq!(
            qb.build().sql(),
            "SELECT * FROM users WHERE TRUE AND name = $1 LIMIT $2 OFFSET $3"
        );
    }
}
