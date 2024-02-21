use docker_compose_types::{BuildStep, Service, Volumes};
use crate::database::help_fns::db_info;

pub fn mariadb(user_name: Option<String>, password: Option<String>, db_name: Option<String>) -> (String, Option<Service>) {
    let build_steps = BuildStep::Simple("mariadb.Dockerfile".to_string());

    let volume = Volumes::Simple("postgres_data:/var/lib/postgresql/data:/app".to_string());

    let volumes = vec![volume];

    let environment = db_info(user_name, password, db_name);

    let service = Some(Service {
        build_: Some(build_steps),
        environment,
        volumes,
        ..Default::default()
    });

    ("mariadb-db".to_string(), service)
}