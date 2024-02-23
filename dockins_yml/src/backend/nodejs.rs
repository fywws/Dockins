use docker_compose_types::{BuildStep, Ports, Service, Volumes};

pub fn nodejs() -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("nodejs.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["3000:3000".to_string()];

    // ADD SUPPORT OF CONFIG
    let volume = Volumes::Simple("./server:/app".to_string());

    let volumes = vec![volume];

    let service = Some(Service {
        build_: Some(build_steps),
        ports: Ports::Short(ports),
        volumes,
        ..Default::default()
    });

    ("nodejs-backend".to_string(), service)
}