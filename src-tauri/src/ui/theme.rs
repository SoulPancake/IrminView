use crate::irmin::types::Theme;

/// Theme configuration utilities
pub struct ThemeManager {
    current_theme: Theme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: Theme::Light,
        }
    }
    
    pub fn set_theme(&mut self, theme: Theme) {
        self.current_theme = theme;
    }
    
    pub fn get_theme(&self) -> &Theme {
        &self.current_theme
    }
    
    pub fn toggle(&mut self) -> &Theme {
        self.current_theme = match self.current_theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
            Theme::System => Theme::Light, // Default to light when toggling from system
        };
        &self.current_theme
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}