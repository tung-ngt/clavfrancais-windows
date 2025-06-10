use tauri::{Builder, Manager, Wry};

use tauri_plugin_autostart::MacosLauncher;

pub trait RegisterPlugins {
    fn register_plugins(self) -> Self;
}

impl RegisterPlugins for Builder<Wry> {
    fn register_plugins(self) -> Self {
        self.plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app_handle, _, _| {
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.show();
            }
        }))
        .plugin(tauri_plugin_shell::init())
    }
}
