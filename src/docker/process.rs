use crate::style_print::*;
use std::str;
use tokio::process::Command;

pub async fn process_docker_build(project_id: &str, service_name: &str, gcr_region: &str) {
    let gcr_url = String::from(gcr_region) + "/" + project_id + "/" + service_name;
    let output = Command::new("docker")
        .args(&["build", ".", "-t", &gcr_url])
        .output()
        .await;
    println!("output = {:?}", output);
}

pub async fn process_docker_push(project_id: &str, service_name: &str, gcr_region: &str) {
    let gcr_url = String::from(gcr_region) + "/" + project_id + "/" + service_name;
    let output = Command::new("docker")
        .args(&["push", &gcr_url])
        .output()
        .await;
    println!("output = {:?}", output);
}

pub async fn create_docker_network() {
    let _output = Command::new("docker")
        .args(&["network", "create", "zapp"])
        .output()
        .await;
}

pub async fn process_docker_restart() {
    let _output = Command::new("docker")
        .args(&["rm", "-f", "zapp-psql"])
        .output()
        .await;
    let _output2 = Command::new("zapp")
        .args(&["docker", "psql"])
        .output()
        .await;
}

pub async fn process_docker_psql(service_name: &str) {
    let underscored_name = service_name.to_string().replace("-", "_");
    let container_name = String::from(service_name) + "-psql";
    let db_name = String::from("POSTGRES_DB=") + &underscored_name + "_db";
    let output = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "-d",
            "--name",
            &container_name,
            "-p",
            "5432:5432",
            "-e",
            "POSTGRES_USER=postgres",
            "-e",
            "POSTGRES_PASSWORD=postgres",
            "-e",
            &db_name,
            "--network=zapp",
            "postgres:14.3-alpine",
        ])
        .output()
        .await;
    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let out = str::from_utf8(&val.stdout);
            match out.unwrap() {
                "" => println!("{:?}", err.unwrap().trim()),
                _ => {
                    log_success(&format!("PostgreSQL Container Created: {}", out.unwrap())).await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}
