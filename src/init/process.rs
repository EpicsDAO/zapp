use tokio::process::Command;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use super::actions_yml::ACTIONS_YML;
use crate::style_print::*;
use regex::Regex;
use std::str;

fn regex(re_str: &str) -> Regex {
  Regex::new(re_str).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
  pub project_id: String,
  pub service_name: String,
  pub region: String,
}

pub async fn process_init_gcp_config() {
  let msg1 = "Please input your GCP project_id:";
  log_input(msg1).await;
  let mut project_id = String::new();
  io::stdin()
    .read_line(&mut project_id)
    .expect("Failed to read line");
  let project_id: String = project_id
    .trim()
    .parse()
    .expect("Please input your GCP project_id:");

  let msg2 = "Please input your GCP service_name:";
  log_input(msg2).await;

  let mut service_name = String::new();
  io::stdin()
    .read_line(&mut service_name)
    .expect("Failed to read line");
  let service_name: String = service_name
    .trim()
    .parse()
    .expect("Please input your GCP service_name:");

  let msg3 = "Please input your GCP region:";
  log_input(msg3).await;
  let mut region = String::new();
  io::stdin()
    .read_line(&mut region)
    .expect("Failed to read line");
  let region: String = region
    .trim()
    .parse()
    .expect("Please input your GCP region:");

  let json_struct = build_gcp_config(project_id, service_name, region).await;
  let result = write_gcp_config(json_struct).await;
  match result {
    Ok(..) => {
      log_success("Successfully Generated!").await;
      log_create_file("File Path: ./gcp_config.json").await;
    }
    Err(err) => {
      log_error(&format!("Failed to Write: {}", err)).await;
    }
  }
}

async fn write_gcp_config(json_struct: GcpConfig) -> std::io::Result<()> {
  let serialized: String = serde_json::to_string_pretty(&json_struct).unwrap();
  let mut file = File::create("gcp_config.json")?;
  file.write_all(serialized.as_bytes())?;
  Ok(())
}

async fn build_gcp_config(project_id: String, service_name: String, region: String) -> GcpConfig {
  GcpConfig {
    project_id,
    service_name,
    region,
  }
}

pub async fn build_api_workflow() {
  let workflow_dir = ".github/workflows";
  fs::create_dir_all(workflow_dir).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  let workflow_yml = ".github/workflows/zapp_service.yml";
  let file_exist = Path::new(workflow_yml).exists();
  match file_exist {
    true => {
      log_error("Error: File already exist!").await;
    }
    false => {
      let mut file = fs::File::create(workflow_yml).unwrap();
      file.write_all(ACTIONS_YML).unwrap();
      log_success("Successfully created workflow!").await;
    }
  }
}

pub async fn dl_zapp(app_name: &str) {
  let version_range = "v0.2";
  let zapp_dl_url = format!("https://storage.googleapis.com/zapp-bucket/zapp-api-template/{}/zapp-api.tar.gz", version_range);
  let output = Command::new("curl")
    .args(&[
      "-OL",
      &zapp_dl_url
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("Received");
      match rt.is_match(err.unwrap()) {
        true => {
          let _ = fs::create_dir(app_name);
          unzip_zapp(app_name).await;
        }
        false => {
          panic!("{:?}", err.unwrap())
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}


pub async fn unzip_zapp(app_name: &str) {
  let filename = "zapp-api.tar.gz";
  let output = Command::new("tar")
    .args(&[
      "-zxvf",
      &filename,
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("could not");
      match rt.is_match(err.unwrap()) {
        true => {
          panic!("{:?}", err.unwrap())
        }
        false => {
          let _ = fs::rename("zapp-api", app_name);
          let _ = fs::remove_file(filename);
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn git_init(app_name: &str) {
  let output = Command::new("cd")
    .args(&[
      &app_name,
      "&&",
      "git",
      "init",
      "--initial-branch=main"
    ])
    .output()
    .await;
  match &output {
    Ok(_val) => {
        // println!("{:?}", val);
      }
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn underscore(s: &str) -> String {
  s.replace("-", "_")
}

pub async fn create_dockerfile(app_name: &str) {
  let filename = format!("{}/Dockerfile", app_name);
  let underscore_app_name = underscore(app_name).await;
  let file_content = format!("FROM rust:1.61 as build
RUN USER=root cargo new --bin {}
WORKDIR /{}
COPY entity entity
COPY migration migration
COPY Cargo.toml Cargo.toml
RUN cargo build --release
COPY ./src ./src
COPY entity/src entity/src
COPY migration/src migration/src
RUN rm -f target/release/deps/{}*
RUN cargo build --release

FROM debian:11.3
COPY --from=build /{}/target/release/{} .

CMD [\"./{}\"]", app_name, app_name, &underscore_app_name, app_name, app_name, app_name);
  let file_exist = Path::new(&filename).exists();
  match file_exist {
    true => {
      log_error("Error: File already exist!").await;
    }
    false => {
      let mut file = fs::File::create(&filename).unwrap();
      file.write_all(file_content.as_bytes()).unwrap();
      log_success("Successfully created Dockerfile!").await;
    }
  }
}

pub async fn endroll(app_name: &str) {
  let text1 = "  ███████╗ █████╗ ██████╗ ██████╗ ";
  let text2 = "  ╚══███╔╝██╔══██╗██╔══██╗██╔══██╗";
  let text3 = "    ███╔╝ ███████║██████╔╝██████╔╝";
  let text4 = "   ███╔╝  ██╔══██║██╔═══╝ ██╔═══╝ ";
  let text5 = "  ███████╗██║  ██║██║     ██║     ";
  let text6 = "  ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝     ";
  log_white(text1).await;
  log_white(text2).await;
  log_white(text3).await;
  log_white(text4).await;
  log_white(text5).await;
  log_white(text6).await;
  log_new(&format!("\nEasy to Run\n$ cd {}\n$ zapp docker psql\n$ cargo run\n\nGo to : http://localhost:3000/api/graphql", app_name)).await;
}
