use clap::{Parser, Subcommand};


const HELP_TEMPLATE:&str = "
\x1b[35mUsage\x1b[0m: {bin}.exe <\x1b[1mCOMMAND\x1b[0m>\n\n\
             \x1b[35mCommands\x1b[0m:\n\
             {subcommands}\n\
             ";

#[derive(Parser)]
#[command(author, help_template = HELP_TEMPLATE)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about="About this CLI")]
    About,

    #[command(alias = "cfg", about="Manage configuration settings")]
    Config,

    #[command(alias = "i", about="Initiates the yml file with frontend (-f), backend (-b), database (-db) server (-s) flags")]
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

    #[command(alias = "fl", about="Prints all supported frontend services. Short form: fl")]
    FrontendList,

    #[command(alias = "bl", about="Prints all supported backend services. Short form: bl")]
    BackendList,

    #[command(alias = "sl", about="Prints all supported web server services. Short form: sl")]
    ServerList,

    #[command(alias = "dbl", about="Prints all supported databases services. Short form: dbl")]
    DatabasesList,


}
