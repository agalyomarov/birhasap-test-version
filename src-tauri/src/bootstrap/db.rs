use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite};
use tauri::Manager;

pub async fn setup(app: &tauri::App) -> sqlx::Pool<Sqlite> {
    let mut path = app
        .path()
        .app_local_data_dir()
        .expect("ошибка при получения пути app_local_data_dir");

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    };

    path.push("db.sqlite");

    Sqlite::create_database(
        format!(
            "sqlite:{}",
            path.to_str().expect("path should be something")
        )
        .as_str(),
    )
    .await
    .expect("failed to create database");

    let db: sqlx::Pool<Sqlite> = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    db
}
