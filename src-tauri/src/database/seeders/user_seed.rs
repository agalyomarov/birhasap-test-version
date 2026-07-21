use crate::entities::{prelude::*, user_entity::ActiveModel};
use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn run(db: &DatabaseConnection) -> Result<()> {
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
