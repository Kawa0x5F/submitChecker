// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod executor;
mod utils;

use std::path::PathBuf;
use tauri::Manager;
use tokio::fs;
use uuid::Uuid;

#[tauri::command]
async fn run_multiple_submissions(
    app: tauri::AppHandle,
    folder_paths: Vec<String>,
    input_file_path: String,
) -> Result<String, String> {
    let mut overall_results = String::new();
    let executor_manager = executor::ExecutorManager::new();

    let input_content = fs::read_to_string(&input_file_path)
        .await
        .map_err(|e| format!("Failed to read input file {}: {}", input_file_path, e))?;

    for (i, folder_path_str) in folder_paths.iter().enumerate() {
        let folder_path = PathBuf::from(folder_path_str);
        let folder_name = folder_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&format!("Folder_{}", i + 1))
            .to_string();

        overall_results.push_str(&format!("--- Running Submission: {} ---\n", folder_name));

        let mut code_file_path_in_folder: Option<PathBuf> = None;
        if folder_path.is_dir() {
            let mut entries = fs::read_dir(&folder_path).await.map_err(|e| {
                format!("Failed to read directory {}: {}", folder_path.display(), e)
            })?;

            // コンパイラの提案に合わせて修正
            while let Ok(Some(entry)) = entries.next_entry().await {
                // entry は tokio::fs::DirEntry 型
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "py" {
                            code_file_path_in_folder = Some(path);
                            break;
                        }
                    }
                }
            }
            // Note: 上記のループでは Result が Err の場合と Option が None の場合はループを抜けます。
            // もし Err の場合に明示的なエラーメッセージを overall_results に追加したい場合は、
            // 以前の match を使ったアプローチの方が適しています。
            // この修正はコンパイルエラーを解消するための最小限の変更です。
        }

        let actual_code_file_path = match code_file_path_in_folder {
            Some(p) => p,
            None => {
                let msg = format!(
                    "Error: No Python file (.py) found in folder: {}\n\n",
                    folder_path.display()
                );
                overall_results.push_str(&msg);
                eprintln!("{}", msg);
                continue;
            }
        };

        let app_data_path = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;

        let temp_subdir_name = format!("submission_{}_{}", folder_name, Uuid::new_v4());
        let temp_dir_path = app_data_path
            .join("temp_submissions")
            .join(temp_subdir_name);

        fs::create_dir_all(&temp_dir_path)
            .await
            .map_err(|e| format!("Failed to create temp dir: {}", e))?;

        let code_temp_file_path = temp_dir_path.join("user_code.py");
        let input_temp_file_path = temp_dir_path.join("input.txt");
        let output_temp_file_path = temp_dir_path.join("output.txt");

        fs::copy(&actual_code_file_path, &code_temp_file_path)
            .await
            .map_err(|e| {
                format!(
                    "Failed to copy code file {}: {}",
                    actual_code_file_path.display(),
                    e
                )
            })?;
        fs::write(&input_temp_file_path, &input_content)
            .await
            .map_err(|e| {
                format!(
                    "Failed to write input temp file {}: {}",
                    input_temp_file_path.display(),
                    e
                )
            })?;

        let execution_result = executor_manager
            .run_code_in_container(
                code_temp_file_path
                    .to_str()
                    .ok_or_else(|| {
                        format!(
                            "Failed to convert path to string: {:?}",
                            code_temp_file_path
                        )
                    })?
                    .to_string(),
                input_temp_file_path
                    .to_str()
                    .ok_or_else(|| {
                        format!(
                            "Failed to convert path to string: {:?}",
                            input_temp_file_path
                        )
                    })?
                    .to_string(),
                output_temp_file_path
                    .to_str()
                    .ok_or_else(|| {
                        format!(
                            "Failed to convert path to string: {:?}",
                            output_temp_file_path
                        )
                    })?
                    .to_string(),
            )
            .await;

        match execution_result {
            Ok(output) => {
                overall_results.push_str(&format!("Result for {}:\n{}\n\n", folder_name, output));
            }
            Err(e) => {
                overall_results.push_str(&format!("Error for {}:\n{}\n\n", folder_name, e));
                eprintln!("Error processing {}: {}", folder_name, e);
            }
        }
        fs::remove_dir_all(&temp_dir_path).await.ok();
    }

    Ok(overall_results)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_multiple_submissions])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
