use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug)]
pub struct DbAuth2;

#[derive(Debug)]
pub struct Auth2 {
    user_id: Uuid,
    pkce: String,
    access_token: String,
    refresh_token: String,
}


impl DbAuth2 {
    pub async fn add_user(pool: &Pool<Postgres>, user_id: Uuid) {
        let user = sqlx::query!(r#"INSERT INTO auth_two (user_id) VALUES ($1) RETURNING user_id"#, user_id).fetch_one(pool).await;

        if let Err(e) = user {
            // 
        }
        // 
    }

    pub async fn update_pkce(pool: &Pool<Postgres>, pkce: String) {
        let res = sqlx::query(r#"UPDATE auth_two SET pkce=$1 WHERE user=$2 RETURNING *"#)
            .bind(pkce)
            .execute(&*pool).await;

        if let Err(e) = res {
            // 
        }
        // 
    }
}