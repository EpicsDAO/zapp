use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use super::actions_yml::ACTIONS_YML;
use crate::style_print::*;

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