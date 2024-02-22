use std::fs::File;
use std::io;
use std::io::Write;
use serde_yaml;
use docker_compose_types::{Compose, Services};

use crate::supported_services::{BackendServices, DatabaseServices, FrontendServices, WebServerServices};

pub fn builder(fe: Option<String>, be: Option<String>, serv: Option<String>, db: Option<String>) {
    // fe - Frontend, be - Backend, serv - web server, db - database
    if fe.is_none() && be.is_none() && serv.is_none() && db.is_none() {
        println!("No parameters provided, please use with --help for correct usage");
        return;
    }

    let mut services = indexmap::IndexMap::new();

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


    if fe.is_some() {
        let arg = fe.clone().unwrap().to_lowercase();
        match FrontendServices::from_arg(arg) {
            None => {
                panic!("The provided frontend argument is not supported or contains errors")
            }
            Some( frontend_context ) => {
                services.insert(frontend_context.0, frontend_context.1)
            }
        };
    };

    if be.is_some() {
        let arg = be.clone().unwrap();
        match BackendServices::from_arg(arg) {
            None => {
                panic!("The provided backend argument is not supported or contains errors")
            }
            Some( backend_context ) => {
                services.insert(backend_context.0, backend_context.1)
            }
        };
    };


    if db.is_some() {
        let arg = db.clone().unwrap();

        let db_args = db_params();

        match DatabaseServices::from_arg(arg, db_args.0, db_args.1, db_args.2) {
            None => {
                panic!("The provided database argument is not supported or contains errors")
            }
            Some( database_context ) => {
                services.insert(database_context.0, database_context.1)
            }
        };
    };

    if serv.is_some() {
        let arg = serv.unwrap();
        match WebServerServices::from_arg(arg, fe, be, db) {
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

fn db_params() -> (Option<String>, Option<String>, Option<String>) {
    let mut username = String::new();
    let mut password = String::new();
    let mut dbname = String::new();

    println!("Enter database username:");
    io::stdin().read_line(&mut username)
        .expect("Failed to read line");

    if username.trim().is_empty() {
        return (None, None, None)
    } else {
        println!("Enter database password:");
        io::stdin().read_line(&mut password)
            .expect("Failed to read line");

        println!("Enter database database name:");
        io::stdin().read_line(&mut dbname)
            .expect("Failed to read line");

        let username = username.trim().to_string();
        let password = password.trim().to_string();
        let dbname = dbname.trim().to_string();

        let username_option = if username.is_empty() { None } else { Some(username) };
        let password_option = if password.is_empty() { None } else { Some(password) };
        if (username_option.is_some() && password_option.is_none()) || (username_option.is_none() && password_option.is_some()) {
            panic!("Please provide both username and password")
        }

        let dbname_option = if dbname.is_empty() { None } else { Some(dbname) };
        return (username_option, password_option, dbname_option)
    }
}