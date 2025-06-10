mod app_state;
mod commands;
mod controllers;
mod language;
mod plugins;
mod settings;
mod setup;
mod toggle_shortcut;
mod tray_menu;
mod utils;
mod window_engine;
mod window_event;

use commands::RegisterCommands;
use plugins::RegisterPlugins;
use setup::AppSetup;
use window_event::ResiterWindowEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_plugins()
        .register_commands()
        .register_window_event()
        .app_setup()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
