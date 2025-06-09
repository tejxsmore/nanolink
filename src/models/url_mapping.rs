use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow::{anyhow, Result};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UrlMapping {
    pub id: Uuid,
    pub short_code: String,
    pub original_url: String,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub click_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlInput {
    pub original_url: String,
    pub short_code: Option<String>,
    pub user_id: Option<Uuid>,
}

pub fn generate_short_code(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub async fn insert_url_mapping(
    pool: &PgPool,
    input: CreateUrlInput,
) -> Result<UrlMapping> {
    let max_retries = 10;
    let mut tries = 0;
    
    // Check if custom short code is requested but user is not authenticated
    if input.short_code.is_some() && input.user_id.is_none() {
        return Err(anyhow!("Custom short codes are only available for authenticated users"));
    }
    
    let short_code = if let Some(code) = input.short_code.clone() {
        // User-defined short code: check existence
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM url_mappings WHERE short_code = $1)"
        )
        .bind(&code)
        .fetch_one(pool)
        .await?;

        if exists {
            return Err(anyhow!("Short code already in use"));
        }
        code
    } else {
        // Auto-generate short code and retry if conflict
        loop {
            if tries >= max_retries {
                return Err(anyhow!("Failed to generate unique short code after several attempts"));
            }
            let code = generate_short_code(6);
            let exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM url_mappings WHERE short_code = $1)"
            )
            .bind(&code)
            .fetch_one(pool)
            .await?;

            if !exists {
                break code;
            }
            tries += 1;
        }
    };

    let id = Uuid::new_v4();

    let rec = sqlx::query_as::<_, UrlMapping>(
        r#"
        INSERT INTO url_mappings (id, short_code, original_url, user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, short_code, original_url, user_id, created_at, expires_at, click_count
        "#
    )
    .bind(id)
    .bind(&short_code)
    .bind(&input.original_url)
    .bind(input.user_id)
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_url_by_code(pool: &PgPool, short_code: &str) -> Result<Option<UrlMapping>, sqlx::Error> {
    let rec = sqlx::query_as::<_, UrlMapping>(
        "SELECT id, short_code, original_url, user_id, created_at, expires_at, click_count FROM url_mappings WHERE short_code = $1"
    )
    .bind(short_code)
    .fetch_optional(pool)
    .await?;

    Ok(rec)
}

pub async fn get_user_urls(pool: &PgPool, user_id: Uuid) -> Result<Vec<UrlMapping>, sqlx::Error> {
    let urls = sqlx::query_as::<_, UrlMapping>(
        "SELECT id, short_code, original_url, user_id, created_at, expires_at, click_count FROM url_mappings WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(urls)
}
