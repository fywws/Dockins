mod cli;
mod supported_services;
mod backend;
mod frontend;
mod server;
mod database;
mod yml_builder;
mod config;
mod config_builder;


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
    
    let mut config = Config::load();
    
    match cli.cmd {
        Commands::Config => {

        }

        Commands::Init { frontend, backend, server, database } => {
            builder(&mut config, frontend, backend, server, database)
        }

        Commands::FrontendList => { supported_frontend_services() }
        Commands::BackendList => { supported_backend_services() }
        Commands::ServerList => { supported_web_server_services() }
        Commands::DatabasesList => { supported_database_services() }

        Commands::About {} => {}
    };
}


