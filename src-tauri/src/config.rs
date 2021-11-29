use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    theme: String, // options: dark, light, custom
}

impl Config {
    pub fn new() -> Self {
        Self {
            theme: "light".to_string()
        }
    }
}
