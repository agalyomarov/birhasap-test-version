pub mod users;

use sqlx::SqlitePool;

pub async fn run(pool: &SqlitePool) {
    users::create_users(pool).await.unwrap();
}
