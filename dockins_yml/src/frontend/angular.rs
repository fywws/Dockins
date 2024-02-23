use docker_compose_types::{BuildStep, Ports, Service, Volumes};

pub fn angular() -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("angular.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["4200:4200".to_string()];

    // ADD SUPPORT OF CONFIG
    let volume = Volumes::Simple("./client:/app".to_string());

    let volumes = vec![volume];

    let service = Some(Service {
        build_: Some(build_steps),
        ports: Ports::Short(ports),
        volumes,
        ..Default::default()
    });

    ("angular-frontend".to_string(), service)
}