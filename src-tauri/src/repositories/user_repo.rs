use sqlx::Error;
use crate::db::connection::get_db_pool;
use crate::models::user::{User, CreateUser, UpdateUser};

pub async fn find_all_users() -> Result<Vec<User>, Error> {
    let pool = get_db_pool().await?;
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn find_user_by_id(user_id: i32) -> Result<Option<User>, Error> {
    let pool = get_db_pool().await?;
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn create_user(new_user: CreateUser) -> Result<User, Error> {
    let pool = get_db_pool().await?;
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, username, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(new_user.name)
    .bind(new_user.username)
    .bind(new_user.email)
    .bind(new_user.password)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn update_user(user_id: i32, updated_user: UpdateUser) -> Result<Option<User>, Error> {
    let pool = get_db_pool().await?;

    let existing_user = find_user_by_id(user_id).await?;
    if existing_user.is_none() {
        return Ok(None);
    }

    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET name = COALESCE($1, name), username = COALESCE($2, username), email = COALESCE($3, email), password = COALESCE($4, password), updated_at = NOW() WHERE id = $5 RETURNING *",
    )
    .bind(updated_user.name)
    .bind(updated_user.username)
    .bind(updated_user.email)
    .bind(updated_user.password)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(Some(user))
}     

pub async fn delete_user(user_id: i32) -> Result<bool, Error> {
    let pool = get_db_pool().await?;
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}