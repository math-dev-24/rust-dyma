use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    image: String,
    ports: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerCompose {
    version: String,
    services: HashMap<String, Service>,
}

pub fn yaml_practice() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("docker-compose.yaml")?;
    let docker_compose: DockerCompose = from_reader(file)?;

    println!("Version: {}", docker_compose.version);

    for (service_name, service) in &docker_compose.services {
        println!("Service: {}", service_name);
        println!("  Image: {}", service.image);
        if let Some(ports) = &service.ports {
            println!("  Ports: {:?}", ports);
        }
    }
    Ok(())
}