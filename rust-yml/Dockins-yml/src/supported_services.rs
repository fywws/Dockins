use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::frontend::{react::react};
use crate::backend::{django::django};
use crate::webserver::{nginx::nginx};

#[derive(Debug, EnumIter)]
pub enum FrontendServices {
    React,
}

impl FrontendServices {
    pub fn from_arg(arg: String) -> Option<()> {
        match arg.as_str() {
            "react" => Some(react()),
            _ => None,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum BackendServices {
    Django
}

impl BackendServices {
    pub fn from_arg(arg: String) -> Option<()> {
        match arg.as_str() {
            "django" => Some(django()),
            _ => None,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum WebServerServices {
    Nginx,
}

impl WebServerServices {
    pub fn from_arg(arg: String) -> Option<()> {
        match arg.as_str() {
            "nginx" => Some(nginx()),
            _ => None,
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum DatabaseServices {
    Postgresql,
}

impl DatabaseServices {
    pub fn from_arg(arg: String) -> Option<()> {
        match arg.as_str() {
            "postgresql" => Some(nginx()),
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

pub fn supported_web_server_services() {
    println!("Supported web server services:");
    for service in WebServerServices::iter() {
        println!("{:?}", service)
    }
}

pub fn supported_database_services() {
    println!("Supported database services:");
    for service in DatabaseServices::iter() {
        println!("{:?}", service)
    }
}