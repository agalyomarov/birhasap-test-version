use uuid::Uuid;

use sqlx::SqlitePool;

pub async fn create_users(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users (id, login, password)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(Uuid::now_v7().to_string())
    .bind("admin")
    .bind("12345")
    .execute(pool)
    .await?;

    Ok(())
}
