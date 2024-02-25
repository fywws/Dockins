use docker_compose_types::{BuildStep, Command, Environment, Ports, Service};
use crate::config::config::Config;
use crate::config::config::ConfigParts::BeCfg;
use crate::config::help_fns::{command, dockerfile_name, env, ports, volumes};

pub fn actix_web(config: &Config) -> (String, Option<Service>) {
    let df_name = match dockerfile_name(config, BeCfg) {
        None => {
            "actix_web.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
        }
    };

    let raw_ports = match ports(config, BeCfg) {
        None => {
            "8080:8080".to_string()
        }
        Some(ports) => {
            ports
        }
    };

    let volumes = volumes(config, BeCfg).unwrap();

    let env = match env(config, BeCfg) {
        Some(env) => {
            Environment::List(env)
        }
        None => {
            Environment::default()
        }
    };

    let raw_command = match command(config, BeCfg) {
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

    let service = Some(Service {
        build_: Some(build_step),
        ports,
        volumes,
        environment,
        command,
        ..Default::default()
    });

    ("actix_web-backend".to_string(), service)
}