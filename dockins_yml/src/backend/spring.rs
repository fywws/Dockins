use docker_compose_types::{BuildStep, Ports, Service, Volumes};

pub fn spring() -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("spring.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["8080:8080".to_string()];

    // ADD SUPPORT OF CONFIG
    let volume = Volumes::Simple("./server:/app".to_string());

    let volumes = vec![volume];

    let service = Some(Service {
        build_: Some(build_steps),
        ports: Ports::Short(ports),
        volumes,
        ..Default::default()
    });

    ("spring-backend".to_string(), service)
}