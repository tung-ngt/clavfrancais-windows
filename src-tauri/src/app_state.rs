use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{debug_println, language::Language, settings::Settings};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppState {
    pub language: Language,
    pub settings: Settings,
}

impl AppState {
    pub fn load(path: PathBuf) -> Self {
        if !path.is_file() {
            let app_data = Self::default();
            let app_data_json = serde_json::to_string(&app_data).unwrap();
            let _ = fs::write(&path, app_data_json);
            return app_data;
        }

        let app_data_json = fs::read_to_string(&path).unwrap();
        let r = serde_json::from_str(&app_data_json);

        if let Ok(settings) = r {
            return settings;
        }

        let app_data = Self::default();
        let app_data_json = serde_json::to_string(&app_data).unwrap();
        let _ = fs::write(&path, app_data_json);
        app_data
    }

    pub fn save(&self, path: PathBuf) {
        let app_data_json = serde_json::to_string(&self).unwrap();
        let r = fs::write(path, app_data_json);
        debug_println!("{:?}", r);
    }
}