// Предотвращает дополнительное окно консоли в выпуске Windows, НЕ УДАЛЯЙТЕ!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axum::{Router, http::StatusCode, response::IntoResponse, routing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }

    let port: u16 = std::env::var("APP_PORT")
        .expect("APP_PORT не установлен")
        .parse()
        .expect("Не удалось спарсить APP_PORT как порт");

    let routes = routes();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("server started on http://127.0.0.1:{}", port);
    axum::serve(listener, routes).await.unwrap();
    Ok(())
}

fn routes() -> Router {
    Router::new().route("/", routing::get(index))
}

async fn index() -> impl IntoResponse {
    let data_dir = std::env::var("APP_DATA_DIR").unwrap_or("APP_DATA_DIR NOT FOUND".to_string());

    (StatusCode::OK, data_dir.to_owned())
}
