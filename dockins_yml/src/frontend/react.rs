use std::vec;
use docker_compose_types::{BuildStep, Command, Environment, Ports, Service, Volumes};
use crate::config::config::Config;
use crate::config::help_fns::{command, dockerfile_name, env, ports, volumes};

pub fn react(config: &Config) -> (String, Option<Service>) {
    let raw_ports = match ports(config) {
        None => {
            if config.be_cfg.is_none() {
              "3000:3000".to_string()
            } else {
                if config.be_cfg.unwrap().framework == "nodejs" {
                    if config.be_cfg.unwrap().ports == Some("3000:3000".to_string()) {
                        println!("Please consider that react has 3001 ports and not 3000");
                        "3001:3001".to_string()
                    } else {
                        "3000:3000".to_string()
                    }
                } else {
                    "3000:3000".to_string()
                }
            }
        }
        Some(port) => {
            port
        }
    };

    let df_name = match dockerfile_name(config) {
        None => {
            "react.Dockerfile".to_string()
        }
        Some(df_name) => {
            df_name
        }
    };

    let volumes = match volumes(config) {
        None => {
            vec![Volumes::Simple("./client:/app".to_string())]
        }
        Some(raw_volumes) => {

            let mut volumes = Vec::with_capacity(8);

            for raw_volume in raw_volumes.iter() {
                let copy_volume = raw_volume.clone();

                volumes.push(
                    Volumes::Simple(copy_volume)
                );
            }

            volumes
        }
    };

    let env = match env(config) {
        Some(env) => {
            Some(Environment::List(env))
        }
        None => {
            None
        }
    };

    let raw_command = match command(config) {
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

    ("react-frontend".to_string(), Some(service))
}