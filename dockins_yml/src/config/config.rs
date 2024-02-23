use std::fs::File;
use crate::config::backend_cfg::BeConfig;
use crate::config::db_cfg::DbConfig;
use crate::config::frontend_cfg::FeConfig;
use crate::config::server_cfg::ServerConfig;


pub struct Config {
    pub fe_cfg: FeConfig,
    pub be_cfg: BeConfig,
    pub db_cfg: DbConfig,
    pub server_cfg: ServerConfig,
}

pub enum ConfigParts {
    FeCfg,
    BeCfg,
    DBCfg,
    ServerCfg,
}

impl Config {
    pub fn new(fe_cfg: FeConfig, be_cfg: BeConfig, db_cfg: DbConfig, server_cfg: ServerConfig) -> Config {
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

    pub fn load() -> Config {
        match File::open("./yml_cfg.toml") {
            Ok(file) => {
                Self {
                    fe_cfg: FeConfig::no_cfg(),
                    be_cfg: BeConfig::no_cfg(),
                    db_cfg: DbConfig::no_cfg(),
                    server_cfg: ServerConfig::no_cfg(),
                }
            }
            Err(_) => {
                Self {
                    fe_cfg: FeConfig::no_cfg(),
                    be_cfg: BeConfig::no_cfg(),
                    db_cfg: DbConfig::no_cfg(),
                    server_cfg: ServerConfig::no_cfg(),
                }
            }
        }
    }
}