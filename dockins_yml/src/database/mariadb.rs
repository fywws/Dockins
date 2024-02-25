use docker_compose_types::{BuildStep, Command, Environment, Ports, Service};
use crate::config::config::Config;
use crate::config::config::ConfigParts::{DBCfg};
use crate::config::help_fns::{command, dockerfile_name, env, ports};

pub fn mariadb(config: &Config) -> (String, Option<Service>) {
    let df_name = match dockerfile_name(config, DBCfg) {
        None => {
            "mariadb.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
        }
    };

    let raw_ports = match ports(config, DBCfg) {
        None => {
            "3306:3306".to_string()
        }
        Some(ports) => {
            ports
        }
    };

    let env = match env(config, DBCfg) {
        Some(env) => {
            Environment::List(env)
        }
        None => {
            Environment::default()
        }
    };

    let raw_command = match command(config, DBCfg) {
        None => {
            None
        }
        Some(command) => {
            Some(command)
        }
    };

    let ports = Ports::Short(vec![raw_ports]);
    let build_step = BuildStep::Simple(df_name);


    let command = match raw_command {
        Some(command) => {
            Some(Command::Simple(command))
        }
        _ => None
    };

    let service = Service {
        build_: Some(build_step),
        ports,
        environment: env,
        command,
        ..Default::default()
    };

    ("mariadb-db".to_string(), Some(service))
}