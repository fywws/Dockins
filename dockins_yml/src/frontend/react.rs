use std::vec;
use docker_compose_types::{BuildStep, Command, Environment, Ports, Service};
use crate::config::config::Config;
use crate::config::config::ConfigParts::FeCfg;
use crate::config::help_fns::{command, dockerfile_name, env, volumes};
use crate::frontend::help_fns::fe_ports;

pub fn react(config: &Config) -> (String, Option<Service>) {

    let port = fe_ports(config, "react");

    let df_name = match dockerfile_name(config, FeCfg) {
        None => {
            "react.Dockerfile".to_string()
        }
        Some(df_name) => {
            if df_name.ends_with(".Dockerfile") {
                df_name
            } else {
                format!("{}.Dockerfile", df_name)
            }
        }
    };

    let volumes = volumes(config, FeCfg).unwrap();

    let env = match env(config, FeCfg) {
        Some(env) => {
            Environment::List(env)
        }
        None => {
            Environment::default()
        }
    };

    let raw_command = match command(config, FeCfg) {
        None => {
            None
        }
        Some(command) => {
            Some(command)
        }
    };

    let ports = Ports::Short(vec![port]);
    let build_step = BuildStep::Simple(df_name);
    let environment = env;
    let command = match raw_command {
        Some(command) => {
            Some(Command::Simple(command))
        }
        _ => None
    };
    let service = Service {
        ports,
        build_: Some(build_step),
        environment,
        volumes,
        command,
        ..Default::default()
    };

    ("react-frontend".to_string(), Some(service))
}