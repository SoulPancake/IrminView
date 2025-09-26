use crate::irmin::types::Theme;
use tauri::command;
use std::sync::atomic::{AtomicBool, Ordering};

static DARK_MODE: AtomicBool = AtomicBool::new(false);

/// Toggle between light and dark theme
#[command]
pub async fn toggle_theme() -> Result<Theme, String> {
    let is_dark = DARK_MODE.load(Ordering::Relaxed);
    let new_theme = if is_dark {
        DARK_MODE.store(false, Ordering::Relaxed);
        Theme::Light
    } else {
        DARK_MODE.store(true, Ordering::Relaxed);
        Theme::Dark
    };
    
    Ok(new_theme)
}

/// Get current theme
#[command]
pub async fn get_current_theme() -> Result<Theme, String> {
    let is_dark = DARK_MODE.load(Ordering::Relaxed);
    Ok(if is_dark { Theme::Dark } else { Theme::Light })
}