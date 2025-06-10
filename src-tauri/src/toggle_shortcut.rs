use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ToggleShortcut {
    #[default]
    CtrlAlt,
    AltZ,
}
