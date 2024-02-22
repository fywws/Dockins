use docker_compose_types::Environment;

pub fn db_info(user_name: Option<String>, password: Option<String>, db_name: Option<String>) -> Environment {
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

        Environment::List(params)
    }

    else {
        Environment::List(vec![
            "POSTGRES_USER=postgres".to_string(),
            "POSTGRES_PASSWORD=postgres".to_string(),
            "POSTGRES_DB=postgres".to_string()
        ])
    }
}