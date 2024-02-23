use docker_compose_types::{BuildStep, Ports, Service, Volumes};

pub fn react() -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("react.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["3000:3000".to_string()];

    // ADD SUPPORT OF CONFIG
    let volume = Volumes::Simple("./client:/app".to_string());

    let volumes = vec![volume];

    let service = Some(Service {
        build_: Some(build_steps),
        ports: Ports::Short(ports),
        volumes,
        ..Default::default()
    });

    ("react-frontend".to_string(), service)
}