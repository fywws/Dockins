use docker_compose_types::{BuildStep, DependsOnOptions, Ports, Service, Volumes};

pub fn dependencies_info(fe: Option<String>, be: Option<String>, db: Option<String>) -> Vec<String> {
    let mut depends_on = vec![];

    if fe.is_some() {
        let fe_dep = format!("{}-frontend", fe.unwrap());
        // dep - dependency
        depends_on.push(fe_dep)
    }

    if be.is_some() {
        let be_dep = format!("{}-backend", be.unwrap());
        depends_on.push(be_dep)
    }

    if db.is_some() {
        let db_dep = format!("{}-db", db.unwrap());
        depends_on.push(db_dep)
    }

    return depends_on;
}

pub fn checks(build_steps: BuildStep, ports: Vec<String>, volumes: Vec<Volumes>, depends_on: Vec<String>) -> (String, Option<Service>) {
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