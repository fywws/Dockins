use docker_compose_types::{Service, BuildStep, Ports, Volumes};

pub fn django() -> (String, Option<Service>) {

    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("django.Dockerfile".to_string());

    // ADD SUPPORT OF CONFIG
    let ports = vec!["8000:8000".to_string()];

    // ADD SUPPORT OF CONFIG
    let volume = Volumes::Simple("./server:/app".to_string());

    let volumes = vec![volume];

    let service = Some(Service {
        build_: Some(build_steps),
        ports: Ports::Short(ports),
        volumes,
        ..Default::default()
    });

    ("django-backend".to_string(), service)
}
