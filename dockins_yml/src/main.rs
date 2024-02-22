mod cli;
mod supported_services;
mod backend;
mod frontend;
mod webserver;
mod database;
mod yml_builder;
mod config;


use clap::{Parser};
use crate::cli::{Cli, Commands};
use crate::supported_services::{
    supported_backend_services,
    supported_database_services,
    supported_frontend_services,
    supported_web_server_services};
use crate::yml_builder::builder;


fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Init { frontend, backend, server, database } => {

            builder(frontend, backend, server, database)
        }

        Commands::FrontendList => { supported_frontend_services() }
        Commands::BackendList => { supported_backend_services() }
        Commands::WebServerList => { supported_web_server_services() }
        Commands::DatabasesList => { supported_database_services() }

        Commands::About {} => {}
    };
    //Add support for users config
}

