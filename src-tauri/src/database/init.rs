use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::path::PathBuf;
use tokio::fs;

use crate::database::{migrate, seeders};

pub async fn init(app_data_path: PathBuf) -> Result<DatabaseConnection> {
    let app_data_path = app_data_path.join("database");
    fs::create_dir_all(&app_data_path).await?;
    let db_path = app_data_path.join("db.sqlite");
    let mut options = ConnectOptions::new(format!("sqlite:{}", db_path.display()));
    options.sqlx_logging(false);

    let db = Database::connect(options).await?;

    migrate::migrate(&db).await?;
    seeders::seeders(&db).await?;
    Ok(db)
}
