use crate::config::config::Config;
use crate::config::config::ConfigParts::FeCfg;

pub fn fe_ports(config: &Config, name: &str) -> String {
    let raw_ports = match crate::config::help_fns::ports(config, FeCfg) {
        None => {
            match config.BackEndConfiguration.is_nodejs {
                None => {
                    "3000:3000".to_string()
                }
                Some(is_nodejs) => {
                    if is_nodejs {
                        println!("please consider that {} has 3001:3001 ports and not 3000:3000", name);
                        "3001:3001".to_string()
                    } else {
                        "3000:3000".to_string()
                    }
                }
            }
        }
        Some(port) => {
            match config.BackEndConfiguration.is_nodejs {
                None => { "3000:3000".to_string() }
                Some(is_nodejs) => {
                    if is_nodejs && port == "3000:3000" {
                        println!("please consider that {} has 3001:3001 ports and not 3000:3000", name);
                        "3001:3001".to_string()
                    } else {
                        port
                    }
                }
            }
        }
    };

    raw_ports
}