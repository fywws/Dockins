
#[derive(Default)]
pub struct BeConfig {
    //pub path_to_dockerfile: Option<String>,
    pub dockerfile_name: Option<String>,
    pub ports: Option<String>,
    pub command: Option<String>,
    pub env: Option<Vec<String>>,
    pub path_to_project: Option<Vec<String>>,
    pub is_nodejs: bool
}

impl BeConfig {
    pub fn no_cfg() -> Self {
        Self {
            ..Default::default()
        }
    }
}