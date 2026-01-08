mod application;
mod domains;
mod presentation;

use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::presentation::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let app: Router = create_router();

    let addr = "0.0.0.0:8085";
    tracing::info!("🚀 Starting TUNIFY backend server");
    tracing::info!("📡 Listening on http://{addr}");
    tracing::info!("📋 Available endpoints:");
    tracing::info!("   GET  http://{addr}/api/v1/status");
    tracing::info!("   GET  http://{addr}/api/v1/health");

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("✅ Server started successfully");
    
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "tunify-backend=info,tower_http=info".into()),
            )
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .compact()
        )
        .init();
    
    tracing::info!("Tracing initialized");
}