use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use crate::config::backend_cfg::BackEndConfig;
use crate::config::db_cfg::DataBaseConfig;
use crate::config::frontend_cfg::FrontEndConfig;
use crate::config::server_cfg::ServerConfig;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub FrontEndConfiguration: FrontEndConfig,
    pub BackEndConfiguration: BackEndConfig,
    pub DataBaseConfiguration: DataBaseConfig,
    pub ServerConfiguration: ServerConfig,
}

pub enum ConfigParts {
    FeCfg,
    BeCfg,
    DBCfg,
    ServerCfg,
}

impl Config {
    pub fn new(fe_cfg: FrontEndConfig, be_cfg: BackEndConfig, db_cfg: DataBaseConfig, server_cfg: ServerConfig) -> Config {
        let config = Self {
            FrontEndConfiguration: fe_cfg,
            BackEndConfiguration: be_cfg,
            DataBaseConfiguration: db_cfg,
            ServerConfiguration: server_cfg,
        };

        config
    }

    pub fn save(self){
        let toml = toml::to_string_pretty(&self).unwrap();

        match File::create("yml_cfg.toml") {
            Ok(mut file) => {
                file.write_all(toml.as_bytes()).expect("Please run as an administrator. If it wont work please contact the developers")
            }
            Err(_) => {
                panic!("Please delete the yml-cfg.toml file. If it doesnt exist please run command as an administrator")
            }
        }

        println!("Successfully saved config")
    }

    pub fn load() -> Config {
        match File::open("./yml_cfg.toml") {
            Ok(mut file) => {
                let mut buffer = [0; 1024];

                match file.read(&mut buffer) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("Unable to read config file. File might be damaged. Please re-create the file")
                    }
                };

                let cfg_string = String::from_utf8(Vec::from(buffer)).unwrap();

                println!("{}", cfg_string);

                let toml_string = toml::from_str(cfg_string.as_str());

                let config: Config = match toml_string {
                    Ok(cfg) => {
                        cfg
                    },
                    Err(err) => panic!("Unable to parse config. Please re-create the config file or fix the errors manually. {}", err)
                };

                config
            }
            Err(_) => {
                Self {
                    FrontEndConfiguration: FrontEndConfig::no_cfg(),
                    BackEndConfiguration: BackEndConfig::no_cfg(),
                    DataBaseConfiguration: DataBaseConfig::no_cfg(),
                    ServerConfiguration: ServerConfig::no_cfg(),
                }
            }
        }
    }
}