mod cli;
mod supported_services;
mod backend;
mod frontend;
mod server;
mod database;
mod yml_builder;
mod config;


use std::fs::File;
use clap::{Parser};
use crate::cli::{Cli, Commands};
use crate::config::config::Config;
use crate::supported_services::{
    supported_backend_services,
    supported_database_services,
    supported_frontend_services,
    supported_web_server_services};
use crate::yml_builder::builder;


fn main() {
    let cli = Cli::parse();
    
    let config = match File::open("./dockins_yml.toml") {
        Ok(cfg_file) => {
            Config::load(cfg_file)
        }
        Err(_) => {
            Config::new(None, None, None, None)
        }
    };
    
    match cli.cmd {
        Commands::Config => {

        }

        Commands::Init { frontend, backend, server, database } => {
            builder(&config, frontend, backend, server, database)
        }

        Commands::FrontendList => { supported_frontend_services() }
        Commands::BackendList => { supported_backend_services() }
        Commands::ServerList => { supported_web_server_services() }
        Commands::DatabasesList => { supported_database_services() }

        Commands::About {} => {}
    };
}


