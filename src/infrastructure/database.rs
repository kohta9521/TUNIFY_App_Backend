use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;
use tracing::info;

/// データベース接続プールを作成
/// 
/// # 引数
/// - `database_url`: PostgreSQL接続URL（例: postgresql://user:pass@host:port/dbname）
/// 
/// # 戻り値
/// - `Result<PgPool>`: 接続プール（成功時）またはエラー
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    println!("📡 Starting database connection...");
    info!("Connecting to database...");
    
    // 接続URLの基本検証
    if !database_url.starts_with("postgresql://") && !database_url.starts_with("postgres://") {
        return Err(anyhow::anyhow!(
            "Invalid DATABASE_URL format. Must start with 'postgresql://' or 'postgres://'. \
            Current value starts with: {}",
            database_url.chars().take(20).collect::<String>()
        ));
    }
    
    // ホスト名を抽出して検証
    if let Some(host_start) = database_url.find("@") {
        let host_part = &database_url[host_start + 1..];
        if let Some(host_end) = host_part.find(":") {
            let hostname = &host_part[..host_end];
            println!("🌐 Connecting to host: {}", hostname);
            info!("Attempting to connect to host: {}", hostname);
            
            if hostname.is_empty() {
                return Err(anyhow::anyhow!("Database hostname is empty in DATABASE_URL"));
            }
        }
    }
    
    println!("⏳ Establishing connection (this may take a few seconds)...");
    let pool = PgPoolOptions::new()
        .max_connections(10)  // 最大接続数
        .connect(database_url)
        .await
        .map_err(|e| {
            println!("❌ Connection failed: {}", e);
            anyhow::anyhow!(
                "Failed to connect to database: {}. \
                Please check:\n\
                1. DATABASE_URL is correct in .env file\n\
                2. Supabase project is active\n\
                3. Network connection is available\n\
                4. Hostname in DATABASE_URL is correct\n\
                5. If IPv4 issue, try Session Pooler (port 6543) instead of Direct connection (port 5432)",
                e
            )
        })?;
    
    println!("✅ Database connection established");
    info!("✅ Database connection established");
    
    // 接続テスト
    println!("🧪 Testing database connection...");
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    
    println!("✅ Database connection test successful");
    info!("✅ Database connection test successful");
    
    Ok(pool)
}

