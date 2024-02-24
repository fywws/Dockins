#[derive(Default)]
pub struct FeConfig {
    //pub path_to_dockerfile: Option<String>,
    pub dockerfile_name: Option<String>,
    pub ports: Option<String>,
    pub command: Option<String>,
    pub env: Option<Vec<String>>,
    pub path_to_project: Option<Vec<String>>,
}

impl FeConfig {
    pub fn no_cfg() -> Self {
        Self {
            ..Default::default()
        }
    }
}