use docker_compose_types::{BuildStep, Command, Environment, Ports, Service, Volumes};
use crate::config::config::Config;
use crate::config::config::ConfigParts::DBCfg;
use crate::config::help_fns::{command, dockerfile_name, env, ports};
use crate::database::help_fns::db_info;

pub fn postgresql(config: &Config) -> (String, Option<Service>) {
    let df_name = match dockerfile_name(config, DBCfg) {
        None => {
            "postgresql.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
        }
    };

    let raw_ports = match ports(config, DBCfg) {
        None => {
            "5432:5432".to_string()
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

    ("postgresql-db".to_string(), Some(service))
}