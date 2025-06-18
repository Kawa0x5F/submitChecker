// リリースビルド時にWindowsで余分なコンソール画面が表示されるのを防ぐ
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;

#[tauri::command]
fn find_folders(parent_folder_path: &str) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(parent_folder_path).map_err(|e| {
        format!(
            "フォルダ '{}' の読み込みに失敗しました: {}",
            parent_folder_path, e
        )
    })?;

    let sub_folders: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect();

    Ok(sub_folders)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_folders])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
