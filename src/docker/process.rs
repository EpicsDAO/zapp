use tokio::process::Command;
use std::str;
use crate::style_print::*;

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


pub async fn process_docker_psql() {
  let output = Command::new("docker")
    .args(&[
      "run",
      "--rm",
      "-d",
      "--name",
      "zapp-psql",
      "-p",
      "5432:5432",
      "-v",
      "postgres-tmp:/home/postgresql/data",
      "-e",
      "POSTGRES_USER=postgres",
      "-e",
      "POSTGRES_PASSWORD=postgres",
      "-e",
      "POSTGRES_DB=zapp_db",
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
    },
    Err(err) => println!("error = {:?}", err)
  }
}
