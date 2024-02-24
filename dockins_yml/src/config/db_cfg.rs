use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[derive(Debug)]
pub struct DataBaseConfig {
    pub dockerfile_name: Option<String>,
    pub ports: Option<String>,
    pub command: Option<String>,
    pub env: Option<Vec<String>>,
}

impl DataBaseConfig {
    pub fn no_cfg() -> Self {
        Self {
            ..Default::default()
        }
    }
}