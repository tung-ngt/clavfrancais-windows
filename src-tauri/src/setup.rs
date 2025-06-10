use std::error::Error;

use crate::{
    app_state::AppState,
    controllers::{start_engine, toggle_language},
    language::Language,
    tray_menu::register_tray_menu,
};

use crate::window_engine::WindowEngine;

use std::sync::{mpsc, Mutex};
use std::thread;
use tauri::Manager;
use tauri::{App, Builder, Emitter, Wry};

pub trait AppSetup {
    fn app_setup(self) -> Self;
}

impl AppSetup for Builder<Wry> {
    fn app_setup(self) -> Self {
        self.setup(setup)
    }
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let app_data_path = app.path().app_config_dir().unwrap();
    let app_state = AppState::load(app_data_path);

    let settings = app_state.settings;

    if !settings.hide_to_tray {
        let _ = app.get_webview_window("main").unwrap().show();
    }

    let app_handle = app.app_handle().clone();

    thread::spawn(move || {
        let (sender, receiver) = mpsc::channel::<()>();
        WindowEngine::set_toggle_channel(sender);

        loop {
            let r = receiver.recv();
            if r.is_err() {
                break;
            }
            toggle_language(&app_handle);
        }
    });

    if app_state.language == Language::French {
        start_engine();
    };

    let _ = app.emit("change_language", app_state.language);

    register_tray_menu(app, app_state.language)?;

    app.manage(Mutex::new(app_state));
    Ok(())
}
