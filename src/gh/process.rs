use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use tokio::process::Command;

pub async fn process_setup_secret() {
  let file_name = ".env";
  let file = File::open(file_name).unwrap();
  let reader = BufReader::new(file);
  let env: EnvConfig = serde_json::from_reader(reader).unwrap();
  println!("env = {:?}", env);
}

pub async fn process_add_env(key: &str, value: &str) {
  let output = Command::new("gh")
    .args(&["secret", "set", key, "-b", value])
    .output()
    .await;
  println!("output = {:?}", output);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConfig {
  pub epics_project_id: String,
  pub epics_service_name: String,
  pub epics_gcp_region: String,
}
