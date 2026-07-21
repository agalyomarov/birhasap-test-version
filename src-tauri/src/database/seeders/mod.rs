use anyhow::{Ok, Result};
use sea_orm::DatabaseConnection;
pub mod user_seed;

pub async fn seeders(db: &DatabaseConnection) -> Result<()> {
    user_seed::run(db).await?;
    Ok(())
}
