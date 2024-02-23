use docker_compose_types::{ Volumes};
use crate::config::config::{Config, ConfigParts};


pub fn ports(config: &Config, config_parts: ConfigParts) -> Option<String> {
    match config_parts {
        ConfigParts::FeCfg => {config.fe_cfg.ports.as_ref().and_then(|ports| Some(ports.clone()))}
        ConfigParts::BeCfg => {config.be_cfg.ports.as_ref().and_then(|ports| Some(ports.clone()))}
        ConfigParts::DBCfg => {config.db_cfg.ports.as_ref().and_then(|ports| Some(ports.clone()))}
        ConfigParts::ServerCfg => {config.server_cfg.ports.as_ref().and_then(|ports| Some(ports.clone()))}
    }
}

pub fn dockerfile_name(config: &Config, config_parts: ConfigParts) -> Option<String> {
    match config_parts {
        ConfigParts::FeCfg => {config.fe_cfg.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
        ConfigParts::BeCfg => {config.be_cfg.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
        ConfigParts::DBCfg => {config.db_cfg.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
        ConfigParts::ServerCfg => {config.server_cfg.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
    }
}

fn cfg_volumes(config: &Config, config_parts: &ConfigParts) -> Option<Vec<String>> {
    match config_parts {
        ConfigParts::FeCfg => {config.fe_cfg.path_to_project.as_ref().and_then(|volumes| Some(volumes.clone()))}
        ConfigParts::BeCfg => {config.be_cfg.path_to_project.as_ref().and_then(|volumes| Some(volumes.clone()))}
        ConfigParts::DBCfg => {return None}
        ConfigParts::ServerCfg => {
            config.server_cfg.path_to_project.as_ref().and_then(|volumes| Some(volumes.clone()))
        }
    }
}

pub fn volumes(config: &Config, config_parts: ConfigParts) -> Option<Vec<Volumes>> {
    let volumes = match cfg_volumes(config, &config_parts) {
        None => {
            match config_parts {
                ConfigParts::FeCfg => {vec![Volumes::Simple("./client:/app".to_string())]}
                ConfigParts::BeCfg => {vec![Volumes::Simple("./server:/app".to_string())]}
                ConfigParts::DBCfg => {return None}
                ConfigParts::ServerCfg => {vec![Volumes::Simple("./web_server:/app".to_string())]}
            }
        }

        Some(raw_volumes) => {
            let mut volumes = Vec::with_capacity(raw_volumes.len());

            for raw_volume in raw_volumes.iter() {
                let copy_volume = raw_volume.clone();

                volumes.push(
                    Volumes::Simple(copy_volume)
                );
            }

            volumes
        }
    };

    Some(volumes)
}


pub fn env(config: &Config, config_parts: ConfigParts) -> Option<Vec<String>> {
    match config_parts {
        ConfigParts::FeCfg => {config.fe_cfg.env.as_ref().and_then(|env| Some(env.clone()))}
        ConfigParts::BeCfg => {config.be_cfg.env.as_ref().and_then(|env| Some(env.clone()))}
        ConfigParts::DBCfg => {config.db_cfg.env.as_ref().and_then(|env| Some(env.clone()))}
        ConfigParts::ServerCfg => {config.server_cfg.env.as_ref().and_then(|env| Some(env.clone()))}
    }
}

pub fn command(config: &Config, config_parts: ConfigParts) -> Option<String> {
    match config_parts {
        ConfigParts::FeCfg => {config.fe_cfg.command.as_ref().and_then(|command| Some(command.clone()))}
        ConfigParts::BeCfg => {config.be_cfg.command.as_ref().and_then(|command| Some(command.clone()))}
        ConfigParts::DBCfg => {config.db_cfg.command.as_ref().and_then(|command| Some(command.clone()))}
        ConfigParts::ServerCfg => {config.server_cfg.command.as_ref().and_then(|command| Some(command.clone()))}
    }
}


