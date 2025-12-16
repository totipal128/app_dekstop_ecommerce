use crate::models::user::{CreateUser, UpdateUser, User};
use crate::repositories::user_repo;

#[tauri::command]
pub async fn get_all_users() -> Result<Vec<User>, String> {
    user_repo::find_all_users().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user_by_id(user_id: i32) -> Result<Option<User>, String> {
    user_repo::find_user_by_id(user_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_new_user(new_user: CreateUser) -> Result<User, String> {
    user_repo::create_user(new_user)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_user_by_id(user_id: i32) -> Result<User, String> {
    let user = user_repo::find_user_by_id(user_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;

    user_repo::delete_user(user_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(user)
}
