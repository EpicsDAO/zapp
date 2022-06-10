use tokio::process::Command;
use std::str;
use console::style;

pub async fn process_docker_build(project_id: &str, service_name: &str) {
  let gcr_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  let output = Command::new("docker")
    .args(&["build", ".", "-t", &gcr_url])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_docker_push(project_id: &str, service_name: &str) {
  let gcr_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  let output = Command::new("docker")
    .args(&["push", &gcr_url])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_docker_psql() {
  let output = Command::new("docker")
    .args(&[
      "run",
      "--rm",
      "-d",
      "--name",
      "epics-psql",
      "-p",
      "5432:5432",
      "-v",
      "postgres-tmp:/home/postgresql/data",
      "-e",
      "POSTGRES_USER=postgres",
      "-e",
      "POSTGRES_PASSWORD=postgres",
      "-e",
      "POSTGRES_DB=epics_db",
      "--network=epics",
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
          println!(
            "✅ {} {}",
            style("PostgreSQL Container Created:").white().bold(),
            style(out.unwrap().trim()).white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}
