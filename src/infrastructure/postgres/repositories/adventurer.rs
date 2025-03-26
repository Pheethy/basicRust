use crate::domain::entities::adventurers::Adventurer;
use crate::domain::repositories::adventurer::AdventurerRepository;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Error, PgPool, Row};
use std::sync::Arc;

pub struct AdventurerPostgres {
    pub db_pool: Arc<PgPool>,
}

impl AdventurerPostgres {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurerRepository for AdventurerPostgres {
    async fn fetch_adventurer_by_id(&self, id: i32) -> Result<Adventurer> {
        let sql = r#"
            SELECT
                to_jsonb("ad")
            FROM (
                SELECT 
                   adventurers.id,
                   adventurers.username,
                   adventurers.password,
                   adventurers.created_at::timestamptz as created_at,
                   adventurers.updated_at::timestamptz as updated_at
                FROM
                    adventurers
                WHERE 
                    adventurers.id = $1
            ) AS "ad"
        "#;

        let row = sqlx::query(sql)
            .bind(id)
            .fetch_one(&*self.db_pool)
            .await
            .map_err(|e| match e {
                Error::RowNotFound => anyhow::anyhow!("Adventurer with id {} not found", id),
                _ => anyhow::Error::new(e),
            })?;

        let json_data: serde_json::Value = row.get("to_jsonb");

        let adventurer = Adventurer {
            id: json_data["id"].as_i64().unwrap() as i32,
            username: json_data["username"].as_str().unwrap().to_string(),
            password: json_data["password"].as_str().unwrap().to_string(),
            created_at: chrono::DateTime::parse_from_rfc3339(
                json_data["created_at"].as_str().unwrap(),
            )
            .unwrap()
            .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(
                json_data["updated_at"].as_str().unwrap(),
            )
            .unwrap()
            .with_timezone(&chrono::Utc),
        };

        Ok(adventurer)
    }
}
