use clap::{Parser, Subcommand};


/// Basic CLI struct
#[derive(Parser)]
#[command(author, version = None, about = None, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initiates the yml file with frontend (-f), backend (-b), database (-db) server (-s) flags

    Init{
        ///Represents the frontend configuration.
        #[arg(required = false, long = "frontend", short)]
        frontend: Option<String>,

        /// Represents the backend configuration.
        #[arg(required = false, long = "backend", short)]
        backend: Option<String>,

        /// Represents the host configuration.
        #[arg(required = false, long = "server", short)]
        server: Option<String>,

        /// Represents the database configuration.
        #[arg(required = false, long = "db", short)]
        database: Option<String>
    },

    /// Prints all supported frontend services. Short form: fl
    #[command(alias = "fl")]
    FrontendList,

    /// Prints all supported backend services. Short form: bl
    #[command(alias = "bl")]
    BackendList,

    /// Prints all supported web server services. Short form: wsl
    #[command(alias = "wsl")]
    WebServerList,

    /// Prints all supported databases services. Short form: dbl
    #[command(alias = "dbl")]
    DatabasesList,

    /// About this CLI

    About,
}
