use crate::config::config::Config;

pub fn ports(config: &Config) -> Option<String> {
    if config.fe_cfg.unwrap().ports.is_some() {
        let ports = config.fe_cfg.unwrap().ports;
        return ports;
    } else { None }
}

pub fn dockerfile_name(config: &Config) -> Option<String> {
    if config.fe_cfg.unwrap().dockerfile_name.is_some() {
        let dockerfile_name = config.fe_cfg.unwrap().dockerfile_name;
        dockerfile_name
    } else { None }
}

pub fn volumes(config: &Config) -> Option<Vec<String>> {
    if config.fe_cfg.unwrap().path_to_project.is_some() {
        let volumes = config.fe_cfg.unwrap().path_to_project;
        volumes
    } else {
        None
    }
}

pub fn env(config: &Config) -> Option<Vec<String>> {
    if config.fe_cfg.unwrap().env.is_some() {
        let env = config.fe_cfg.unwrap().env;

        env
    } else {
        None
    }
}

pub fn command(config: &Config) -> Option<String> {
    if config.fe_cfg.unwrap().command.is_some() {
        let command = config.fe_cfg.unwrap().command;

        command
    } else {
        None
    }
}

