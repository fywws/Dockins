use docker_compose_types::{BuildStep, Environment, Service, Volumes};

pub fn postgresql(user_name: Option<String>, password: Option<String>, db_name: Option<String>) -> (String, Option<Service>) {
    // ADD SUPPORT OF CONFIG
    let build_steps = BuildStep::Simple("posgresql.Dockerfile".to_string());

    let volume = Volumes::Simple("postgres_data:/var/lib/postgresql/data:/app".to_string());

    let volumes = vec![volume];

    let mut environment;

    if user_name.is_some() && password.is_some() {
        let mut params = vec![
            format!("POSTGRES_USER={}", user_name.unwrap()),
            format!("POSTGRES_PASSWORD={}", password.unwrap()),
        ];

        if db_name.is_some() {
            params.push(format!("POSTGRES_DB={}", db_name.unwrap()));
        } else {
            params.push("POSTGRES_DB=postgres".to_string())
        }

        environment = Environment::List(params);
    }

    else {
        environment = Environment::List(vec![
            "POSTGRES_USER=postgres".to_string(),
            "POSTGRES_PASSWORD=postgres".to_string(),
            "POSTGRES_DB=postgres".to_string()
        ]);
    }

    let service = Some(Service {
        build_: Some(build_steps),
        environment,
        volumes,
        ..Default::default()
    });

    ("postgres-db".to_string(), service)
}