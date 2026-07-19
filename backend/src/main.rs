use axum::{Router, http::StatusCode, response::IntoResponse, routing};

#[tokio::main]
async fn main() {
    let routes = routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("server started on http://0.0.0.0:8000");
    axum::serve(listener, routes).await.unwrap();
}

fn routes() -> Router {
    Router::new().route("/", routing::get(index))
}

async fn index() -> impl IntoResponse {
    (StatusCode::OK, "Hello world".to_string())
}
