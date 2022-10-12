use super::actions_yml::*;
use crate::style_print::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
    pub project_id: String,
    pub service_name: String,
    pub region: String,
    pub network: String,
}

pub fn process_init_gcp_config() {
    let msg1 = "Please Input Your GCP Project ID:";
    log_input(msg1);
    let mut project_id = String::new();
    io::stdin()
        .read_line(&mut project_id)
        .expect("Failed to read line");
    let project_id: String = project_id
        .trim()
        .parse()
        .expect("Please Input Your GCP Project ID:");

    let msg2 = "Please Input Your GCP Service Name:";
    log_input(msg2);

    let mut service_name = String::new();
    io::stdin()
        .read_line(&mut service_name)
        .expect("Failed to read line");
    let service_name: String = service_name
        .trim()
        .parse()
        .expect("Please input your GCP service_name:");

    let msg3 = "Please Input Your GCP Region:";
    log_input(msg3);
    let mut region = String::new();
    io::stdin()
        .read_line(&mut region)
        .expect("Failed to read line");
    let region: String = region
        .trim()
        .parse()
        .expect("Please Input Your GCP Region:");

    let msg4 = "Please Input Your GCP Network:";
    log_input(msg4);
    let mut network = String::new();
    io::stdin()
        .read_line(&mut network)
        .expect("Failed to read line");
    let network: String = network
        .trim()
        .parse()
        .expect("Please input your GCP Network:");

    let json_struct = build_gcp_config(project_id, service_name, region, network);
    let result = write_gcp_config(json_struct);
    match result {
        Ok(..) => {
            log_success("Successfully Generated!");
            log_create_file("File Path: ./gcp_config.json");
        }
        Err(err) => {
            log_error(&format!("Failed to Write: {}", err));
        }
    }
}

fn write_gcp_config(json_struct: GcpConfig) -> std::io::Result<()> {
    let serialized: String = serde_json::to_string_pretty(&json_struct).unwrap();
    let mut file = File::create("gcp_config.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn build_gcp_config(
    project_id: String,
    service_name: String,
    region: String,
    network: String,
) -> GcpConfig {
    GcpConfig {
        project_id,
        service_name,
        region,
        network,
    }
}

pub fn build_api_workflow(gcr_region: &str) {
    let workflow_dir = ".github/workflows";
    fs::create_dir_all(workflow_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let workflow_yml = ".github/workflows/zapp_service.yml";
    let file_exist = Path::new(workflow_yml).exists();
    match file_exist {
        true => {
            log_error("Error: File already exist!");
        }
        false => {
            let mut file = fs::File::create(workflow_yml).unwrap();
            file.write_all(action_yml(gcr_region).as_bytes()).unwrap();
            log_success("Successfully created workflow!");
        }
    }
}

pub fn git_init(app_name: &str) {
    let output = Command::new("cd")
        .args(&[&app_name, "&&", "git", "init", "--initial-branch=main"])
        .output();

    match &output {
        Ok(_val) => {
            // println!("{:?}", val);
        }
        Err(err) => println!("error = {:?}", err),
    }
}
