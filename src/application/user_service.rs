use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;

use crate::domains::user::User;

/// ユーザーサービスの実装
#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    /// 新しいUserServiceインスタンスを作成
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// ユーザーIDでユーザー情報を取得
    /// 
    /// # 引数
    /// - `user_id`: ユーザーID（UUID）
    /// 
    /// # 戻り値
    /// - `Result<Option<User>>`: ユーザー情報（見つからない場合はNone）
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// メールアドレスでユーザー情報を取得
    /// 
    /// # 引数
    /// - `email`: メールアドレス
    /// 
    /// # 戻り値
    /// - `Result<Option<User>>`: ユーザー情報（見つからない場合はNone）
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// すべてのユーザーを取得（デバッグ用）
    /// 
    /// # 戻り値
    /// - `Result<Vec<User>>`: ユーザーリスト
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}

