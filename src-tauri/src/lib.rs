// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod controller;
mod db;
mod models;
mod repositories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::block_on(async {
                // Initialize the database pool
                db::connection::get_db_pool().await;
                db::connection::run_migrations().await;
            });
            // You can perform additional setup here if needed
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            controller::user_controller::get_all_users,
            controller::user_controller::get_user_by_id,
            controller::user_controller::create_new_user,
            controller::user_controller::delete_user_by_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
