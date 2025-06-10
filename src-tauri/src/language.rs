use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    French,
}