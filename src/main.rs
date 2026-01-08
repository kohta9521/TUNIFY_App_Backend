mod application;
mod domains;
mod infrastructure;
mod presentation;

use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

use crate::infrastructure::database::create_pool;
use crate::presentation::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .envファイルを読み込む
    match dotenv() {
        Ok(_) => println!("✅ .env file loaded"),
        Err(e) => println!("⚠️  Warning: Could not load .env file: {}", e),
    }
    
    println!("🔧 Initializing tracing...");
    init_tracing();
    println!("✅ Tracing initialized");
    
    // すぐにログを表示して、プログラムが動いていることを確認
    tracing::info!("🚀 Application starting...");
    println!("🔍 Checking DATABASE_URL...");

    // データベース接続プールを作成
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => {
            println!("✅ DATABASE_URL found (length: {} chars)", url.len());
            // セキュリティのため、パスワード部分をマスクして表示
            let masked_url = if let Some(at_pos) = url.find('@') {
                if let Some(colon_pos) = url[..at_pos].rfind(':') {
                    format!("{}:***@{}", &url[..colon_pos+1], &url[at_pos+1..])
                } else {
                    "***".to_string()
                }
            } else {
                "***".to_string()
            };
            
            tracing::info!("DATABASE_URL found: {}", masked_url);
            
            // 接続URLの基本検証
            if !url.starts_with("postgresql://") && !url.starts_with("postgres://") {
                tracing::error!("DATABASE_URL must start with 'postgresql://' or 'postgres://'");
                tracing::error!("Current value starts with: {}", url.chars().take(20).collect::<String>());
                return Err(anyhow::anyhow!("Invalid DATABASE_URL format"));
            }
            
            // ホスト名を抽出
            if let Some(host_start) = url.find("@") {
                let host_part = &url[host_start + 1..];
                if let Some(host_end) = host_part.find(":") {
                    let hostname = &host_part[..host_end];
                    tracing::info!("Attempting to connect to host: {}", hostname);
                    
                    if hostname.is_empty() {
                        tracing::error!("Database hostname is empty in DATABASE_URL");
                        return Err(anyhow::anyhow!("Database hostname is empty"));
                    }
                } else {
                    tracing::error!("Invalid DATABASE_URL format: missing port number");
                    return Err(anyhow::anyhow!("Invalid DATABASE_URL format: missing port"));
                }
            } else {
                tracing::error!("Invalid DATABASE_URL format: missing '@' symbol");
                return Err(anyhow::anyhow!("Invalid DATABASE_URL format: missing '@'"));
            }
            
            url
        }
        Err(e) => {
            println!("❌ DATABASE_URL not found: {}", e);
            eprintln!("❌ ERROR: DATABASE_URL environment variable is not set!");
            eprintln!("Please create a .env file in the project root with:");
            eprintln!("  DATABASE_URL=postgresql://postgres:kohta0421901!@db.cwltpqqirdxuespxogmg.supabase.co:5432/postgres");
            tracing::error!("DATABASE_URL environment variable is not set!");
            tracing::error!("Please create a .env file in the project root with:");
            tracing::error!("  DATABASE_URL=postgresql://postgres:password@db.xxxxx.supabase.co:5432/postgres");
            return Err(anyhow::anyhow!(
                "DATABASE_URL environment variable must be set. \
                Please create a .env file in the project root directory."
            ));
        }
    };
    
    println!("🔌 Attempting to connect to database...");
    let pool = match create_pool(&database_url).await {
        Ok(pool) => {
            println!("✅ Database connection pool created successfully");
            pool
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            eprintln!("💡 Please check:");
            eprintln!("   1. DATABASE_URL is correct in .env file");
            eprintln!("   2. Supabase project is active");
            eprintln!("   3. Network connection is available");
            eprintln!("   4. If IPv4 issue, try Session Pooler instead");
            return Err(e);
        }
    };

    // ルーターを作成（データベースプールを渡す）
    println!("🔧 Creating router...");
    let app: Router = create_router(pool).await;
    println!("✅ Router created successfully");

    let addr = "0.0.0.0:8085";
    println!("🚀 Starting TUNIFY backend server");
    tracing::info!("🚀 Starting TUNIFY backend server");
    println!("📡 Listening on http://{}", addr);
    tracing::info!("📡 Listening on http://{addr}");
    println!("📋 Available endpoints:");
    tracing::info!("📋 Available endpoints:");
    println!("   GET  http://{}/api/v1/status", addr);
    println!("   GET  http://{}/api/v1/health", addr);
    println!("   GET  http://{}/api/v1/users/{{user_id}}", addr);
    tracing::info!("   GET  http://{addr}/api/v1/status");
    tracing::info!("   GET  http://{addr}/api/v1/health");
    tracing::info!("   GET  http://{addr}/api/v1/users/{{user_id}}");

    println!("🔌 Binding to {}...", addr);
    let listener = TcpListener::bind(addr).await?;
    println!("✅ Server started successfully");
    tracing::info!("✅ Server started successfully");
    println!("🎉 Server is ready to accept connections!");
    
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