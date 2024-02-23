use std::fs::File;
use crate::config::backend_cfg::BeConfig;
use crate::config::db_cfg::DbConfig;
use crate::config::frontend_cfg::FeConfig;
use crate::config::server_cfg::ServerConfig;

#[derive(Default)]
pub struct Config {
    pub fe_cfg: Option<FeConfig>,
    pub be_cfg: Option<BeConfig>,
    pub db_cfg: Option<DbConfig>,
    pub server_cfg: Option<ServerConfig>,
}


impl Config {
    pub fn new(fe_cfg: Option<FeConfig>, be_cfg: Option<BeConfig>, db_cfg: Option<DbConfig>, server_cfg: Option<ServerConfig>) -> Config {
        let config = Self {
            fe_cfg,
            be_cfg,
            db_cfg,
            server_cfg,
        };

        config
    }

    pub fn save(self) -> Config {
        self
    }

    pub fn load(file: File) -> Config {
        Self {
            fe_cfg: None,
            be_cfg: None,
            db_cfg: None,
            server_cfg: None,
        }
    }
}