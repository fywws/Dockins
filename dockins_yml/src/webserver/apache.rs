use docker_compose_types::{BuildStep, Service, Volumes};
use crate::webserver::help_fns::{checks, dependencies_info};

pub fn apache(fe: Option<String>, be: Option<String>, db: Option<String>) -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("apache.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["80:80".to_string()];

    // ADD SUPPORT OF CONFIG

    let volume1 = Volumes::Simple("./apache-config:/usr/local/apache2/conf".to_string());
    let volume2 = Volumes::Simple("./apache-logs:/usr/local/apache2/logs".to_string());

    let volumes = vec![volume1, volume2];

    let depends_on = dependencies_info(fe, be, db);

    checks(build_steps, ports, volumes, depends_on)
}