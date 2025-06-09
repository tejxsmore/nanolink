use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

pub async fn create_user(pool: &PgPool, input: CreateUserInput) -> Result<User> {
    // Check if username or email already exists
    let existing_user = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)"
    )
    .bind(&input.username)
    .bind(&input.email)
    .fetch_one(pool)
    .await?;

    if existing_user {
        return Err(anyhow!("Username or email already exists"));
    }

    // Hash the password
    let password_hash = hash(input.password.as_bytes(), DEFAULT_COST)
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

    let id = Uuid::new_v4();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(&input.username)
    .bind(&input.email)
    .bind(&password_hash)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn authenticate_user(pool: &PgPool, input: LoginInput) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&input.username)
    .fetch_optional(pool)
    .await?;

    match user {
        Some(user) => {
            if verify(&input.password, &user.password_hash)
                .map_err(|e| anyhow!("Password verification failed: {}", e))? {
                Ok(user)
            } else {
                Err(anyhow!("Invalid credentials"))
            }
        }
        None => Err(anyhow!("Invalid credentials")),
    }
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
