use docker_compose_types::{BuildStep, Command, Environment, Ports, Service, Volumes};
use crate::config::config::Config;
use crate::config::config::ConfigParts::BeCfg;
use crate::config::help_fns::{command, dockerfile_name, env, ports, volumes};

pub fn spring(config: &Config) -> (String, Option<Service>) {
    let df_name = match dockerfile_name(config, BeCfg) {
        None => {
            "spring.Dockerfile".to_string()
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
            Some(Environment::List(env))
        }
        None => {
            None
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
    let environment = env.unwrap();
    let command = Some(Command::Simple(raw_command.unwrap()));

    let service = Some(Service {
        build_: Some(build_step),
        ports,
        volumes,
        environment,
        command,
        ..Default::default()
    });

    ("spring-backend".to_string(), service)
}