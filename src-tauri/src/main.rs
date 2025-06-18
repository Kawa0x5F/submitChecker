// リリースビルド時にWindowsで余分なコンソール画面が表示されるのを防ぐ
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

#[tauri::command]
fn find_folders(parent_folder_path: &str) -> Vec<String> {
    vec![parent_folder_path.to_string()]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
