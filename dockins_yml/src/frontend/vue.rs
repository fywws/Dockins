use docker_compose_types::{BuildStep, Command, Environment, Ports, Service};
use crate::config::config::Config;
use crate::config::config::ConfigParts::FeCfg;
use crate::config::help_fns::{command, dockerfile_name, env, volumes};
use crate::frontend::help_fns::fe_ports;

pub fn vue(config: &Config)  -> (String, Option<Service>) {
    let raw_ports = fe_ports(config, "vue");

    let df_name = match dockerfile_name(config, FeCfg) {
        None => {
            "react.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
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

    let ports = Ports::Short(vec![raw_ports]);
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

    ("vue-frontend".to_string(), Some(service))
}