use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // ルーター定義
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler));

    // 0.0.0.0:3000 で待ち受け
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");
    println!("listening on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn root_handler() -> &'static str {
    "TUNIFY backend is running"
}

async fn hello_handler() -> &'static str {
    "Hello from Rust!"
}
