use docker_compose_types::{BuildStep, Command, Environment, Ports, Service};
use crate::config::config::Config;
use crate::config::config::ConfigParts::FeCfg;
use crate::config::help_fns::{command, dockerfile_name, env, ports, volumes};

pub fn angular(config: &Config) -> (String, Option<Service>) {
    let raw_ports = match ports(config, FeCfg) {
        None => {
            "4200:4200".to_string()
        }
        Some(ports) => {
            ports
        }
    };

    let df_name = match dockerfile_name(config, FeCfg) {
        None => {
            "angular.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
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

    let service = Service {
        ports,
        build_: Some(build_step),
        environment,
        volumes,
        command,
        ..Default::default()
    };

    ("angular-frontend".to_string(), Some(service))
}