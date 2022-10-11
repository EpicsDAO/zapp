use crate::style_print::*;
use std::process::Command;
use std::str;

pub fn process_psql_docker() {
    create_docker_network();
    process_docker_psql("zapp-psql");
}

pub fn process_docker_build(project_id: &str, service_name: &str, gcr_region: &str) {
    let gcr_url = String::from(gcr_region) + "/" + project_id + "/" + service_name;
    let output = Command::new("docker")
        .args(&["build", ".", "-t", &gcr_url])
        .output();

    println!("output = {:?}", output);
}

pub fn process_docker_push(project_id: &str, service_name: &str, gcr_region: &str) {
    let gcr_url = String::from(gcr_region) + "/" + project_id + "/" + service_name;
    let output = Command::new("docker").args(&["push", &gcr_url]).output();

    println!("output = {:?}", output);
}

fn create_docker_network() {
    let _output = Command::new("docker")
        .args(&["network", "create", "zapp"])
        .output();
}

pub fn process_docker_restart() {
    let _output = Command::new("docker")
        .args(&["rm", "-f", "zapp-psql"])
        .output();

    let _output2 = Command::new("zapp").args(&["docker", "psql"]).output();
}

fn process_docker_psql(service_name: &str) {
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
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let out = str::from_utf8(&val.stdout);
            match out.unwrap() {
                "" => println!("{:?}", err.unwrap().trim()),
                _ => {
                    log_success(&format!("PostgreSQL Container Created: {}", out.unwrap()));
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}
