use tauri::{Builder, WindowEvent, Wry};

pub trait ResiterWindowEvent {
    fn register_window_event(self) -> Self;
}

impl ResiterWindowEvent for Builder<Wry> {
    fn register_window_event(self) -> Self {
       self.on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
    }
}
