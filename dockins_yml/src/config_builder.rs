use inquire::MultiSelect;

use crate::config::config::Config;
use crate::config::db_cfg::DataBaseConfig;
use crate::config::backend_cfg::BackEndConfig;
use crate::config::frontend_cfg::FrontEndConfig;
use crate::config::server_cfg::ServerConfig;
use crate::config_subbuilders::{backend_cfg_builder, db_cfg_builder, frontend_cfg_builder, server_cfg_builder};


pub fn cfg_builder(frontend: bool, backend: bool, server: bool, database: bool) {
    let mut fe_cfg = FrontEndConfig::no_cfg();
    let mut be_cfg = BackEndConfig::no_cfg();
    let mut db_cfg = DataBaseConfig::no_cfg();
    let mut server_cfg = ServerConfig::no_cfg();

    if !frontend && !backend && !server && !database {
        full_cfg_builder(&mut fe_cfg, &mut be_cfg, &mut server_cfg, &mut db_cfg)
    }

    if frontend {
        println!("Frontend Configuration");
        frontend_cfg_builder(&mut fe_cfg)
    }

    if backend {
        println!("Backend Configuration");
        backend_cfg_builder(&mut be_cfg)
    }

    if server {
        println!("Server Configuration");
        server_cfg_builder(&mut server_cfg)
    }

    if database {
        println!("Database Configuration");
        db_cfg_builder(&mut db_cfg)
    }

    let config = Config::new(fe_cfg, be_cfg, db_cfg, server_cfg);

    config.save()
}


fn full_cfg_builder(fe_config: &mut FrontEndConfig, be_cfg: &mut BackEndConfig, server_config: &mut ServerConfig, db_config: &mut DataBaseConfig) {
    let options = vec![
        "Frontend",
        "Backend",
        "Server",
        "Database",
    ];

    let answers = MultiSelect::new("Select the services you want to configure", options).prompt().unwrap();

    for answer in answers.iter() {
        if answer == &"Frontend" {
            println!("Frontend Configuration");
            frontend_cfg_builder(fe_config)
        }

        if answer == &"Backend" {
            println!("Backend Configuration");
            backend_cfg_builder(be_cfg)
        }

        if answer == &"Server" {
            println!("Server Configuration");
            server_cfg_builder(server_config)
        }

        if answer == &"Database" {
            println!("Database Configuration");
            db_cfg_builder(db_config)
        }
    }
}