use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

// use crate::entities::user;

pub async fn users(db: &DatabaseConnection) -> Result<()> {
    // let count = user::Entity::find().count(db).await?;

    // if count > 0 {
    //     return Ok(());
    // }

    // for (login, password) in [("admin", "12345"), ("kassir", "12345")] {
    //     user::ActiveModel {
    //         uuid: Set(Uuid::now_v7().to_string()),
    //         login: Set(login.into()),
    //         password: Set(password.into()),
    //         ..Default::default()
    //     }
    //     .insert(db)
    //     .await?;
    // }

    Ok(())
}
