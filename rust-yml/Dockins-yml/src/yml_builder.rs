use std::fs::File;
use serde_yaml;
use docker_compose_types::{Compose, Service, Services, SingleService};

use crate::supported_services::{BackendServices, DatabaseServices, FrontendServices, WebServerServices};

pub fn builder(fe: Option<String>, be_df: Option<String>, serv: Option<String>, db: Option<String>) {
    // fe - Frontend, be - Backend, serv - web server, db - database
    if fe.is_none() && be_df.is_none() && serv.is_none() && db.is_none() {
        println!("No parameters provided, please use with --help for correct usage");
        return;
    }

    let mut services = indexmap::IndexMap::new();

    println!("Creating file in current directory...");

    let file = match File::create("./docker-compose.yml") {
        Ok(file) => {
            file
        }
        Err(_) => {
            println!("Error creating file");
            return;
        }
    };


    if fe.is_some() {
        let arg = fe.unwrap();
        match FrontendServices::from_arg(arg) {
            None => {
                panic!("The provided frontend argument is not supported or contains errors")
            }
            Some(..) => {}
        }
    }

    if be_df.is_some() {
        let arg = be_df.unwrap();
        match BackendServices::from_arg(arg) {
            None => {
                panic!("The provided backend argument is not supported or contains errors")
            }
            Some(..) => {}
        }
    }

    if serv.is_some() {
        let arg = serv.unwrap();
        match WebServerServices::from_arg(arg) {
            None => {
                panic!("The provided backend argument is not supported or contains errors")
            }
            Some(..) => {}
        }
    }

    if db.is_some() {
        let arg = db.unwrap();

        match DatabaseServices::from_arg(arg) {
            None => {
                panic!("The provided backend argument is not supported or contains errors")
            }
            Some(..) => {}
        }
    }

    let mut content = Compose {
        version: Some("3.8".to_string()),
        services : {
            Services(services)
        },
        ..Default::default()
    };
}