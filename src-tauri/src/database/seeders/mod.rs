use anyhow::{Ok, Result};
use sea_orm::DatabaseConnection;
pub mod users;

pub async fn seeders(db: &DatabaseConnection) -> Result<()> {
    users::users(db).await?;
    Ok(())
}
