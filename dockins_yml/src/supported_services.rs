use docker_compose_types::Service;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::frontend::{react::react, angular::angular, vue::vue};
use crate::backend::{django::django, spring::spring, nodejs::nodejs};
use crate::backend::actix_web::actix_web;
use crate::config::config::Config;
use crate::database::{postgresql::postgresql, mariadb::mariadb};
use crate::server::{nginx::nginx, apache::apache};

#[derive(Debug, EnumIter)]
pub enum FrontendServices {
    React,
    Angular,
    Vue
}

impl FrontendServices {
    pub fn from_arg(arg: String, config:&Config) -> Option<(String, Option<Service>)> {
        match arg.as_str() {
            "react" => Some(react(config)),
            "angular" => Some(angular(config)),
            "vue" => Some(vue(config)),
            _ => None,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum BackendServices {
    Django,
    Spring,
    Nodejs,
    ActixWeb
}

impl BackendServices {
    pub fn from_arg(arg: String, config:&Config) -> Option<(String, Option<Service>)> {
        match arg.as_str() {
            "django" => Some(django(config)),
            "spring" => Some(spring(config)),
            "nodejs" => Some(nodejs(config)),
            "actixweb" => Some(actix_web(config)),
            _ => None,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum DatabaseServices {
    Postgresql,
    Mariadb
}

impl DatabaseServices {
    pub fn from_arg(arg: String,  config:&Config) -> Option<(String, Option<Service>)> {
        match arg.as_str() {
            "postgresql" => Some(postgresql(config)),
            "mariadb" => Some(mariadb(config)),
            _ => None,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum ServerServices {
    Nginx,
    Apache
}

impl ServerServices {
    pub fn from_arg(arg: String, fe: Option<String>, be: Option<String>, db: Option<String>, config:&Config) -> Option<(String, Option<Service>)> {
        match arg.as_str() {
            "nginx" => Some(nginx(fe, be, db)),
            "apache" => Some(apache(fe, be, db)),
            _ => None,
        }
    }
}

pub fn supported_frontend_services() {
    println!("Supported frontend services:");
    for service in FrontendServices::iter() {
        println!("{:?}", service)
    }
}

pub fn supported_backend_services() {
    println!("Supported backend services:");
    for service in BackendServices::iter() {
        println!("{:?}", service)
    }
}

pub fn supported_database_services() {
    println!("Supported database services:");
    for service in DatabaseServices::iter() {
        println!("{:?}", service)
    }
}

pub fn supported_web_server_services() {
    println!("Supported web server services:");
    for service in ServerServices::iter() {
        println!("{:?}", service)
    }
}
