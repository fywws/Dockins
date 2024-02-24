use docker_compose_types::{BuildStep, Command, Environment, Ports, Service};
use crate::config::config::Config;
use crate::config::config::ConfigParts::FeCfg;
use crate::config::help_fns::{command, dockerfile_name, env, ports, volumes};

pub fn nodejs(config:&Config) -> (String, Option<Service>) {
    let df_name = match dockerfile_name(config, FeCfg) {
        None => {
            "django.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
        }
    };

    let raw_ports = match ports(config, FeCfg) {
        None => {
            "3000:3000".to_string()
        }
        Some(ports) => {
            ports
        }
    };

    let volumes = volumes(config, FeCfg).unwrap();

    let env = match env(config, FeCfg) {
        Some(env) => {
            Some(Environment::List(env))
        }
        None => {
            None
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

    ("nodejs-backend".to_string(), service)
}