use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ServerConfig {
    pub dockerfile_name: Option<String>,
    pub ports: Option<String>,
    pub command: Option<String>,
    pub env: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
}

impl ServerConfig {
    pub fn no_cfg() -> Self {
        Self {
            ..Default::default()
        }
    }
}