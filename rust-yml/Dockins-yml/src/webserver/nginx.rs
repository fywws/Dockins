use std::fmt::format;
use docker_compose_types::{BuildStep, DependsOnOptions, Ports, Service, Volumes};

pub fn nginx(fe: Option<String>, be: Option<String>, db: Option<String>) -> (String, Option<Service>) {

    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("nginx.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["80:80".to_string()];

    // ADD SUPPORT OF CONFIG

    let volume = Volumes::Simple("./nginx/conf.d:/etc/nginx/conf.d".to_string());

    let volumes = vec![volume];

    let mut depends_on = vec![];

    if fe.is_some() {
        let fe_dep = format!("{}-frontend",fe.unwrap());
        // dep - dependency
        depends_on.push(fe_dep)
    }

    if be.is_some() {
        let be_dep = format!("{}-backend", be.unwrap());
        depends_on.push(be_dep)
    }

    if db.is_some() {
        let db_dep = format!("{}-db",db.unwrap());
        depends_on.push(db_dep)
    }

    if depends_on.len() != 0 {
        let fixed = DependsOnOptions::Simple(depends_on);

        let service = Some(Service {
            build_: Some(build_steps),
            ports: Ports::Short(ports),
            volumes,
            depends_on: fixed,
            ..Default::default()
        });
        ("nginx-webserver".to_string(), service)

    } else {
        let service = Some(Service {
            build_: Some(build_steps),
            ports: Ports::Short(ports),
            volumes,
            ..Default::default()
        });
        ("nginx-webserver".to_string(), service)

    }




}