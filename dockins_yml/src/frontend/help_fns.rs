use crate::config::config::Config;
use crate::config::config::ConfigParts::FeCfg;

pub fn fe_ports(config: &Config) -> String {
    let raw_ports = match crate::config::help_fns::ports(config, FeCfg) {
        None => {
            if config.be_cfg.is_nodejs {
                println!("please consider that react has 3001:3001 ports and not 3000:3000");
                "3001:3001".to_string()
            } else {
                "3000:3000".to_string()
            }
        }
        Some(port) => {
            if config.be_cfg.is_nodejs && port == "3000:3000"{
                println!("please consider that react has 3001:3001 ports and not 3000:3000");
                "3001:3001".to_string()
            } else {
                port
            }
        }
    };

    raw_ports
}