use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct BackEndConfig {
    pub dockerfile_name: Option<String>,
    pub ports: Option<String>,
    pub command: Option<String>,
    pub env: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    #[serde(skip_serializing)]
    pub is_nodejs: Option<bool>
}

impl BackEndConfig {
    pub fn no_cfg() -> Self {
        Self {
            ..Default::default()
        }
    }
}