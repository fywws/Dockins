
use docker_compose_types::{BuildStep, Service, Volumes};
use crate::webserver::help_fns::{checks, dependencies_info};

pub fn nginx(fe: Option<String>, be: Option<String>, db: Option<String>) -> (String, Option<Service>) {

    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("nginx.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["80:80".to_string()];

    // ADD SUPPORT OF CONFIG

    let volume = Volumes::Simple("./nginx/conf.d:/etc/nginx/conf.d".to_string());

    let volumes = vec![volume];

    let depends_on = dependencies_info(fe, be, db);

    checks(build_steps, ports, volumes, depends_on)
}