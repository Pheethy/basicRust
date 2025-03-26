use crate::domain::entities::adventurers::Adventurer;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait AdventurerRepository {
    async fn fetch_adventurer_by_id(&self, id: i32) -> Result<Adventurer>;
}
