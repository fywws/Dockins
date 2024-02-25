use std::fs::File;

use std::io::Write;
use serde_yaml;
use docker_compose_types::{Compose, Service, Services};
use indexmap::IndexMap;
use inquire::{MultiSelect, Select};

use crate::config::config::Config;

use crate::supported_services::{BackendServices, DatabaseServices, FrontendServices, ServerServices};

pub fn builder(config:&mut Config, fe: Option<String>, be: Option<String>, serv: Option<String>, db: Option<String>) {
    // fe - Frontend, be - Backend, serv - web server, db - database

    let mut services = indexmap::IndexMap::new();

    if fe.is_none() && be.is_none() && serv.is_none() && db.is_none() {
        full_yml_builder(config, &mut services);
    }

    println!("Creating file in current directory...");

    let mut file = match File::create("./docker-compose.yml") {
        Ok(file) => {
            file
        }
        Err(_) => {
            println!("Error creating file");
            return;
        }
    };

    if be.is_some() {
        let arg = be.clone().unwrap().to_lowercase();
        if arg == "nodejs" {
            config.BackEndConfiguration.is_nodejs = Some(true);
        }
        match BackendServices::from_arg(arg, config) {
            None => {
                panic!("The provided backend argument is not supported or contains errors")
            }
            Some( backend_context ) => {
                services.insert(backend_context.0, backend_context.1)
            }
        };
    };

    if fe.is_some() {
        let arg = fe.clone().unwrap().to_lowercase();
        match FrontendServices::from_arg(arg, config) {
            None => {
                panic!("The provided frontend argument is not supported or contains errors")
            }
            Some( frontend_context ) => {
                services.insert(frontend_context.0, frontend_context.1)
            }
        };
    };

    if db.is_some() {
        let arg = db.clone().unwrap().to_lowercase();

        match DatabaseServices::from_arg(arg, config) {
            None => {
                panic!("The provided database argument is not supported or contains errors")
            }
            Some( database_context ) => {
                services.insert(database_context.0, database_context.1)
            }
        };
    };

    if serv.is_some() {
        let arg = serv.unwrap().to_lowercase();
        match ServerServices::from_arg(arg, fe, be, db, config) {
            None => {
                panic!("The provided web server argument is not supported or contains errors")
            }
            Some( webserver_context ) => {
                services.insert(webserver_context.0, webserver_context.1)
            }
        };
    };

    let content = Compose {
        version: Some("3.8".to_string()),
        services : {
            Services(services)
        },
        ..Default::default()
    };

    let serialized = match serde_yaml::to_string(&content) {
        Ok(s) => s,
        Err(e) => panic!("Failed to serialize docker-compose file: {}", e),
    };

    file.write_all(serialized.as_ref()).unwrap()
}

fn full_yml_builder(config: &Config, services: &mut IndexMap<String, Option<Service>>) {
    let options = vec![
        "Frontend",
        "Backend",
        "Database",
        "Server",
    ];

    let mut fe: Option<String> = None;
    let mut be: Option<String> = None;
    let mut db: Option<String> = None;

    let answers = MultiSelect::new("Select the services you would like to add to your compose file", options).prompt().unwrap();

    for answer in answers.iter() {
        if answer == &"Frontend" {
            let frontend_options = vec![
                "React",
                "Angular",
                "Vue"
            ];

            let answer = Select::new("Select the frontend service", frontend_options).prompt().unwrap();

            fe = Some(answer.to_string().to_lowercase());

            match FrontendServices::from_arg(answer.to_string().to_lowercase(), config) {
                None => {
                    panic!("The provided frontend argument is not supported or contains errors")
                }
                Some( frontend_context ) => {
                    services.insert(frontend_context.0, frontend_context.1)
                }
            };
        }

        if answer == &"Backend" {
            let backend_options = vec![
                "Django",
                "Spring",
                "Nodejs",
                "ActixWeb"
            ];


            let answer = Select::new("Select the backend service", backend_options).prompt().unwrap();

            be = Some(answer.to_string().to_lowercase());

            match BackendServices::from_arg(answer.to_string().to_lowercase(), config) {
                None => {
                    panic!("The provided frontend argument is not supported or contains errors")
                }
                Some(backend_context) => {
                    services.insert(backend_context.0, backend_context.1)
                }
            };
        }

        if answer == &"Database" {
            let database_options = vec![
                "Postgresql",
                "Mariadb"
            ];

            let answer = Select::new("Select the database service", database_options).prompt().unwrap();

            db = Some(answer.to_string().to_lowercase());

            match DatabaseServices::from_arg(answer.to_string().to_lowercase(), config) {
                None => {
                    panic!("The provided database argument is not supported or contains errors")
                }
                Some( database_context ) => {
                    services.insert(database_context.0, database_context.1)
                }
            };
        }

        if answer == &"Server" {
            let server_options = vec![
                "Apache",
                "Nginx"
            ];

            let answer = Select::new("Select the server service", server_options).prompt().unwrap();


            match ServerServices::from_arg(answer.to_string().to_lowercase(), fe.clone(), be.clone(),db.clone(), config) {
                None => {
                    panic!("The provided web server argument is not supported or contains errors")
                }
                Some( webserver_context ) => {
                    services.insert(webserver_context.0, webserver_context.1)
                }
            };
        }
    }
}