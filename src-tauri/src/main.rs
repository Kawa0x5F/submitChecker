// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod executor;
mod utils;

use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
async fn run_submission(
    app: tauri::AppHandle,
    code_content: String,
    input_content: String,
) -> Result<String, String> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?; // tauri::Error を String に変換

    let temp_subdir_name = format!("submission_{}", Uuid::new_v4());
    let temp_dir_path = app_data_path
        .join("temp_submissions")
        .join(temp_subdir_name);

    tokio::fs::create_dir_all(&temp_dir_path)
        .await
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let code_file_path = temp_dir_path.join("user_code.py");
    let input_file_path = temp_dir_path.join("input.txt");
    let output_file_path = temp_dir_path.join("output.txt");

    tokio::fs::write(&code_file_path, code_content)
        .await
        .map_err(|e| format!("Failed to write code file: {}", e))?;
    tokio::fs::write(&input_file_path, input_content)
        .await
        .map_err(|e| format!("Failed to write input file: {}", e))?;

    let executor_manager = executor::ExecutorManager::new();

    let execution_result = executor_manager
        .run_code_in_container(
            code_file_path
                .to_str()
                .ok_or("Failed to convert code_file_path to str")?
                .to_string(),
            input_file_path
                .to_str()
                .ok_or("Failed to convert input_file_path to str")?
                .to_string(),
            output_file_path
                .to_str()
                .ok_or("Failed to convert output_file_path to str")?
                .to_string(),
        )
        .await;

    execution_result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_submission])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_os::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
