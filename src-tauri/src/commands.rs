use std::sync::Mutex;

use tauri::{AppHandle, Builder, Manager, Wry};

use crate::{
    app_state::AppState,
    controllers::{change_language, disable_run_on_startup, enable_run_on_startup, quit, toggle_language},
    language::Language,
    settings::Settings,
};

#[tauri::command]
pub fn get_language_command(app_handle: AppHandle) -> Language {
    let app_state = app_handle.state::<Mutex<AppState>>();

    let language = if let Ok(app_state) = app_state.lock() {
        app_state.language
    } else {
        Language::English
    };

    language
}

#[tauri::command]
pub fn change_language_command(app_handle: AppHandle, language: Language) {
    change_language(&app_handle, language);
}

#[tauri::command]
pub fn toggle_language_command(app_handle: AppHandle) {
    toggle_language(&app_handle);
}

#[tauri::command]
pub fn quit_command(app_handle: AppHandle) {
    quit(&app_handle);
}

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> Settings {
    let app_state = app_handle.state::<Mutex<AppState>>();
    let app_state = app_state.lock().unwrap();
    app_state.settings
}

#[tauri::command]
pub fn set_settings_command(app_handle: AppHandle, settings: Settings) {
    let app_state = app_handle.state::<Mutex<AppState>>();
    let mut app_state = app_state.lock().unwrap();
    app_state.settings = settings;

    if settings.run_on_startup {
        enable_run_on_startup(&app_handle);
    } else {
        disable_run_on_startup(&app_handle);
    }

    let path = app_handle.path().app_config_dir().unwrap();
    app_state.save(path);
}

pub trait RegisterCommands {
    fn register_commands(self) -> Self;
}

impl RegisterCommands for Builder<Wry> {
    fn register_commands(self) -> Self {
        self.invoke_handler(tauri::generate_handler![
            get_language_command,
            change_language_command,
            quit_command,
            get_settings_command,
            set_settings_command,
            toggle_language_command
        ])
    }
}
