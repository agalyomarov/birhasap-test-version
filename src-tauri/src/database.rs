use crate::entities::prelude::*;
use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::EntityTrait;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, PaginatorTrait, Set,
};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

use crate::entities::user_entity::ActiveModel;

pub async fn init(app_data_path: PathBuf) -> Result<DatabaseConnection> {
    let app_data_path = app_data_path.join("database");
    fs::create_dir_all(&app_data_path).await?;
    let db_path = app_data_path.join("db.sqlite");
    let mut options = ConnectOptions::new(format!("sqlite:{}", db_path.display()));
    options.sqlx_logging(false);

    let db = Database::connect(options).await?;

    migrate(&db).await?;
    seeders(&db).await?;
    Ok(db)
}

async fn migrate(db: &DatabaseConnection) -> Result<()> {
    Migrator::up(db, None).await?;
    Ok(())
}

pub async fn seeders(db: &DatabaseConnection) -> Result<()> {
    user_seed(db).await?;
    Ok(())
}

async fn user_seed(db: &DatabaseConnection) -> Result<()> {
    let count = UserEntity::find().count(db).await?;
    if count > 0 {
        return Ok(());
    }
    for (login, password) in [("admin", "12345"), ("kassir", "12345")] {
        ActiveModel {
            uuid: Set(Uuid::now_v7().to_string()),
            login: Set(login.into()),
            password: Set(password.into()),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    Ok(())
}
