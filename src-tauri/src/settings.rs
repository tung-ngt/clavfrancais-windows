use serde::{Deserialize, Serialize};

use crate::toggle_shortcut::ToggleShortcut;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub run_on_startup: bool,
    pub hide_to_tray: bool,
    pub toggle_shortcut: ToggleShortcut,
}
