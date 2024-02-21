use docker_compose_types::{BuildStep, Environment, Service, Volumes};
use crate::database::help_fns::db_info;

pub fn postgresql(user_name: Option<String>, password: Option<String>, db_name: Option<String>) -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("posgresql.Dockerfile".to_string());

    let volume = Volumes::Simple("postgres_data:/var/lib/postgresql/data:/app".to_string());

    let volumes = vec![volume];

    let environment = db_info(user_name, password, db_name);

    let service = Some(Service {
        build_: Some(build_steps),
        environment,
        volumes,
        ..Default::default()
    });

    ("postgres-db".to_string(), service)
}