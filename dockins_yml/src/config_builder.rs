use crate::config::config::Config;
use crate::config::db_cfg::DbConfig;
use crate::config::backend_cfg::BeConfig;
use crate::config::frontend_cfg::FeConfig;
use crate::config::server_cfg::ServerConfig;

pub fn cfg_builder(frontend: bool, backend: bool, server: bool, database: bool) {
    let mut fe_cfg = FeConfig::no_cfg();
    let mut be_cfg = BeConfig::no_cfg();
    let mut db_cfg = DbConfig::no_cfg();
    let mut server_cfg = ServerConfig::no_cfg();

    if !frontend && !backend && !server && !database{
        Config::new(fe_cfg, be_cfg, db_cfg, server_cfg);
        return;
    }

    if frontend {

    }

    if backend {

    }

    if server {

    }

    if database {

    }

    let config = Config::new(fe_cfg, be_cfg, db_cfg, server_cfg);

    config.save()
}


fn full_cfg_builder() {

}