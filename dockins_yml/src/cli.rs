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
    #[command(alias = "cfg", about="Manage configuration settings. If flags are not specified the builder will include all configuration")]
    Config {
        /// Includes the frontend configuration in builder
        #[arg(required = false, long = "frontend", short)]
        frontend: bool,

        /// Includes the backend configuration in builder
        #[arg(required = false, long = "backend", short)]
        backend: bool,

        /// Includes the webserver configuration in builder
        #[arg(required = false, long = "server", short)]
        server: bool,

        /// Includes the database configuration in builder
        #[arg(required = false, long = "db", short)]
        database: bool
    },

    #[command(alias = "i", about="Initiates the yml file with frontend (-f), backend (-b), database (-db) server (-s) flags")]
    Init{
        /// Allows to pick the frontend technology.
        #[arg(required = false, long = "frontend", short)]
        frontend: Option<String>,

        /// Allows to pick the backend technology.
        #[arg(required = false, long = "backend", short)]
        backend: Option<String>,

        /// Allows to pick the webserver technology.
        #[arg(required = false, long = "server", short)]
        server: Option<String>,

        /// Allows to pick the database technology.
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
