// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod irmin;
mod ui;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            irmin::commands::get_tree,
            irmin::commands::get_commits,
            irmin::commands::get_branches,
            irmin::commands::get_commit_diff,
            irmin::commands::search_keys,
            irmin::commands::connect_to_irmin_store,
            irmin::commands::check_irmin_availability,
            ui::commands::toggle_theme
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
