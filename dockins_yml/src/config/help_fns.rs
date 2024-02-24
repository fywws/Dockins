use docker_compose_types::{ Volumes};
use crate::config::config::{Config, ConfigParts};


pub fn ports(config: &Config, config_parts: ConfigParts) -> Option<String> {
    match config_parts {
        ConfigParts::FeCfg => {config.FrontEndConfiguration.ports.as_ref().and_then(|ports| Some(ports.clone()))}
        ConfigParts::BeCfg => {config.BackEndConfiguration.ports.as_ref().and_then(|ports| Some(ports.clone()))}
        ConfigParts::DBCfg => {config.DataBaseConfiguration.ports.as_ref().and_then(|ports| Some(ports.clone()))}
        ConfigParts::ServerCfg => {config.ServerConfiguration.ports.as_ref().and_then(|ports| Some(ports.clone()))}
    }
}

pub fn dockerfile_name(config: &Config, config_parts: ConfigParts) -> Option<String> {
    match config_parts {
        ConfigParts::FeCfg => {config.FrontEndConfiguration.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
        ConfigParts::BeCfg => {config.BackEndConfiguration.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
        ConfigParts::DBCfg => {config.DataBaseConfiguration.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
        ConfigParts::ServerCfg => {config.ServerConfiguration.dockerfile_name.as_ref().and_then(|dockerfile_name| Some(dockerfile_name.clone()))}
    }
}

pub fn correct_dockerfile_name(df_name: String) -> String {
    if df_name.ends_with(".Dockerfile") {
        df_name
    } else {
        format!("{}.Dockerfile", df_name)
    }
}

fn cfg_volumes(config: &Config, config_parts: &ConfigParts) -> Option<Vec<String>> {
    match config_parts {
        ConfigParts::FeCfg => {config.FrontEndConfiguration.volumes.as_ref().and_then(|volumes| Some(volumes.clone()))}
        ConfigParts::BeCfg => {config.BackEndConfiguration.volumes.as_ref().and_then(|volumes| Some(volumes.clone()))}
        ConfigParts::DBCfg => {return None}
        ConfigParts::ServerCfg => {
            config.ServerConfiguration.volumes.as_ref().and_then(|volumes| Some(volumes.clone()))
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
        ConfigParts::FeCfg => {config.FrontEndConfiguration.env.as_ref().and_then(|env| Some(env.clone()))}
        ConfigParts::BeCfg => {config.BackEndConfiguration.env.as_ref().and_then(|env| Some(env.clone()))}
        ConfigParts::DBCfg => {config.DataBaseConfiguration.env.as_ref().and_then(|env| Some(env.clone()))}
        ConfigParts::ServerCfg => {config.ServerConfiguration.env.as_ref().and_then(|env| Some(env.clone()))}
    }
}

pub fn command(config: &Config, config_parts: ConfigParts) -> Option<String> {
    match config_parts {
        ConfigParts::FeCfg => {config.FrontEndConfiguration.command.as_ref().and_then(|command| Some(command.clone()))}
        ConfigParts::BeCfg => {config.BackEndConfiguration.command.as_ref().and_then(|command| Some(command.clone()))}
        ConfigParts::DBCfg => {config.DataBaseConfiguration.command.as_ref().and_then(|command| Some(command.clone()))}
        ConfigParts::ServerCfg => {config.ServerConfiguration.command.as_ref().and_then(|command| Some(command.clone()))}
    }
}


