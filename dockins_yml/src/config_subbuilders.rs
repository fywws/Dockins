use inquire::{InquireError, Select, Text};
use crate::config::backend_cfg::BackEndConfig;
use crate::config::db_cfg::DataBaseConfig;
use crate::config::frontend_cfg::FrontEndConfig;
use crate::config::server_cfg::ServerConfig;

pub fn cfg_builder<T, F>(config: &mut T, builder_fn: F)
    where
        F: FnOnce() -> Result<Option<T>, ()>,
{
    if let Ok(Some(result)) = builder_fn() {
        *config = result;
    }
}

pub fn frontend_cfg_builder(fe_config: &mut FrontEndConfig) {
    cfg_builder(fe_config, || Ok(Some(FrontEndConfig {
        dockerfile_name: dockerfile_name().ok(),
        ports: ports().ok(),
        command: command().ok(),
        volumes: path_to_project().ok(),
        env: env().ok(),
    })));
}

pub fn backend_cfg_builder(be_config: &mut BackEndConfig) {
    cfg_builder(be_config, || Ok(Some(BackEndConfig {
        dockerfile_name: dockerfile_name().ok(),
        ports: ports().ok(),
        command: command().ok(),
        volumes: path_to_project().ok(),
        env: env().ok(),
        is_nodejs: false
    })));
}

pub fn server_cfg_builder(server_config: &mut ServerConfig) {
    cfg_builder(server_config, || Ok(Some(ServerConfig {
        dockerfile_name: dockerfile_name().ok(),
        ports: ports().ok(),
        command: command().ok(),
        volumes: path_to_project().ok(),
        env: env().ok(),
    })));
}

pub fn db_cfg_builder(db_config: &mut DataBaseConfig) {
    cfg_builder(db_config, || Ok(Some(DataBaseConfig {
        dockerfile_name: dockerfile_name().ok(),
        ports: ports().ok(),
        command: command().ok(),
        env: env().ok(),
    })));
}

fn dockerfile_name() -> Result<String, ()> {
    let dockerfile_name = Text::new("Please provide the name of the dockerfile\n").prompt();

    match dockerfile_name {
        Ok(dockerfile_name) => {
            return Ok(dockerfile_name);
        }
        Err(_) => Err(println!("An error happened when asking for dockerfile name, try again later.")),
    }
}

fn ports() -> Result<String, ()> {
    let ports = Text::new("Please provide the ports\n").prompt();

    match ports {
        Ok(ports) => {
            Ok(ports)
        }
        Err(_) => Err(println!("An error happened when asking for ports, try again later.")),
    }
}

fn command() -> Result<String, ()> {
    let command = Text::new("Please provide the command\n").prompt();

    match command {
        Ok(command) => {
            return Ok(command);
        }
        Err(_) => Err(println!("An error happened when asking for command, try again later.")),
    }
}

fn env() -> Result<Vec<String>, ()> {
    let mut env_vars: Vec<String> = vec![];
    repeat_env(&mut env_vars).unwrap();


    loop {
        let options: Vec<&str> = vec!["Yes", "No"];
        let ans: Result<&str, InquireError> = Select::new("Would you like to add another environment variable?\n", options).prompt();

        match ans {
            Ok(choice) => {
                match choice {
                    "Yes" => {
                        repeat_env(&mut env_vars).unwrap();
                    },
                    _ => {break}
                }
            }
            Err(_) => return Err(println!("There was an error, please try again")),
        }
    }


    Ok(env_vars)
}

fn repeat_env(env_vars: &mut Vec<String>)-> Result<(), ()> {
    let env = Text::new("Please provide the environment with next syntax name:value\n").prompt();

    match env {
        Ok(environment) => {
            env_vars.push(environment.to_uppercase());
            return Ok(());
        }
        Err(_) => Err(println!("An error happened when asking for command, try again later.")),
    }
}

fn path_to_project() -> Result<Vec<String>, ()> {
    let volume = Text::new("Please provide the path to project\n").prompt();


    let mut volumes = Vec::with_capacity(4);

    match volume {
        Ok(volume) => {
            volumes.push(volume);
            Ok(volumes)
        }
        Err(_) => Err(println!("An error happened when asking for command, try again later.")),
    }
}