use crate::gh::process_setup_secret;
use crate::style_print::*;
use regex::Regex;
use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct EnvProduction {
    pub database_url: String,
    pub zapp_gcloudsql_instance: String,
    pub zapp_gcp_project_id: String,
    pub zapp_service_name: String,
    pub zapp_gcp_region: String,
    pub zapp_network_name: String,
    pub zapp_env: String,
}

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

pub fn process_create_sql(project_id: &str, service_name: &str, region: &str, network: &str) {
    log_input("Please input your DB Root Password:");
    let mut db_password = String::new();
    io::stdin()
        .read_line(&mut db_password)
        .expect("Failed to read line");
    let db_password: String = db_password
        .trim()
        .parse()
        .expect("Please input DB Root Password:");
    let zone = String::from(region) + "-b";
    log_time("Createting Cloud SQL ...\nThis process takes 5 to 10 min.");
    let instance_name = String::from(service_name) + "-db";
    let db_version = String::from("--database-version=POSTGRES_14");
    let output = Command::new("gcloud")
        .args(&[
            "sql",
            "instances",
            "create",
            &instance_name,
            &db_version,
            "--cpu=1",
            "--memory=4096MB",
            "--zone",
            &zone,
            "--root-password",
            &db_password,
            "--database-flags",
            "cloudsql.iam_authentication=on",
            "--project",
            project_id,
        ])
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("ERROR:");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => log_success("Successfully created Cloud SQL!"),
            }
        }
        Err(err) => {
            println!("error = {:?}", err)
        }
    }
    let internal_ip = get_instance_ip(project_id, service_name, 1);
    let database_url = String::from("DATABASE_URL=postgres://postgres:")
        + &db_password
        + "@"
        + &internal_ip
        + ":5432/"
        + &instance_name
        + "\n";
    let zapp_gcloudsql_instance = String::from("ZAPP_GCLOUDSQL_INSTANCE=\"")
        + &project_id
        + ":"
        + &region
        + ":"
        + &instance_name
        + "\"\n";
    let zapp_gcp_project_id = String::from("ZAPP_GCP_PROJECT_ID=") + &project_id + "\n";
    let zapp_service_name = String::from("ZAPP_SERVICE_NAME=") + &service_name + "\n";
    let zapp_gcp_region = String::from("ZAPP_GCP_REGION=") + &region + "\n";
    let zapp_network_name = String::from("ZAPP_NETWORK_NAME=") + &network + "\n";
    let zapp_env = String::from("ZAPP_ENV=production") + "\n";
    let env_production = EnvProduction {
        database_url,
        zapp_gcloudsql_instance,
        zapp_gcp_project_id,
        zapp_service_name,
        zapp_gcp_region,
        zapp_network_name,
        zapp_env,
    };
    let filename = ".env.production";
    let mut file = fs::File::create(filename).unwrap();
    file.write_all(env_production.database_url.as_bytes())
        .unwrap();
    file.write_all(env_production.zapp_gcloudsql_instance.as_bytes())
        .unwrap();
    file.write_all(env_production.zapp_gcp_project_id.as_bytes())
        .unwrap();
    file.write_all(env_production.zapp_service_name.as_bytes())
        .unwrap();
    file.write_all(env_production.zapp_gcp_region.as_bytes())
        .unwrap();
    file.write_all(env_production.zapp_env.as_bytes()).unwrap();
    process_setup_secret();
}

pub fn process_patch_sql(project_id: &str, service_name: &str, action: &str) {
    let instance_name = String::from(service_name) + "-db";
    let activation_policy = match action {
        "start" => "ALWAYS",
        "stop" => "NEVER",
        _ => {
            panic!("No action name!");
        }
    };

    log_time("Patching Cloud SQL ...\nThis process takes 5 to 10 min.");
    let output = Command::new("gcloud")
        .args(&[
            "sql",
            "instances",
            "patch",
            &instance_name,
            "--activation-policy",
            activation_policy,
            "--project",
            project_id,
        ])
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("ERROR:");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => log_success("Successfully patched Cloud SQL!"),
            }
        }
        Err(err) => {
            println!("error = {:?}", err)
        }
    }
}

pub fn process_restart_sql(project_id: &str, service_name: &str) {
    let instance_name = String::from(service_name) + "-db";
    let output = Command::new("gcloud")
        .args(&[
            "sql",
            "instances",
            "restart",
            &instance_name,
            "--project",
            project_id,
        ])
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("ERROR:");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => log_success("Successfully restart Cloud SQL!"),
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub fn process_create_ip_range(project_id: &str, service_name: &str) {
    let ip_range_name = String::from(service_name) + "-ip-range";
    let network = String::from("--network=") + service_name;
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "addresses",
            "create",
            &ip_range_name,
            "--global",
            "--purpose=VPC_PEERING",
            "--prefix-length=16",
            "--description='peering range for Epics'",
            &network,
            "--project",
            project_id,
        ])
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("ERROR:");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => log_success("Successfully created IP range!"),
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub fn process_connect_vpc_connector(project_id: &str, service_name: &str) {
    let ip_range_name = String::from(service_name) + "-ip-range";
    let network = String::from("--network=") + service_name;
    let output = Command::new("gcloud")
        .args(&[
            "services",
            "vpc-peerings",
            "connect",
            "--service=servicenetworking.googleapis.com",
            "--ranges",
            &ip_range_name,
            &network,
            "--project",
            project_id,
        ])
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("ERROR:");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => log_success("Successfully connected to VPC!"),
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub fn process_assign_network(project_id: &str, service_name: &str) {
    log_time("Assign network ...\nThis process takes 5 to 10 min.");
    let instance_name = String::from(service_name) + "-db";
    let network = String::from("--network=") + service_name;
    let output = Command::new("gcloud")
        .args(&[
            "beta",
            "sql",
            "instances",
            "patch",
            &instance_name,
            &network,
            "--project",
            project_id,
        ])
        .output();

    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stderr);
            let rt = regex("ERROR:");
            match rt.is_match(err.unwrap()) {
                true => {
                    panic!("{:?}", err.unwrap())
                }
                false => log_success("Successfully Assigned Network!"),
            }
        }
        Err(err) => {
            println!("error = {:?}", err)
        }
    }
}

pub fn get_instance_ip(project_id: &str, service_name: &str, ip_type: usize) -> String {
    let instance_name = String::from(service_name) + "-db";
    let mut _internal_ip = String::new();
    let output = Command::new("gcloud")
        .args(&[
            "sql",
            "instances",
            "describe",
            &instance_name,
            "--project",
            project_id,
            "--format",
            "value(ipAddresses.ipAddress)",
        ])
        .output();

    let _internal_ip = match &output {
        Ok(val) => {
            let ips = str::from_utf8(&val.stdout).unwrap().trim();
            let ip_array = ips.split(';').fold(Vec::new(), |mut s, i| {
                s.push(i.to_string());
                s
            });
            if ip_type == 0 {
                ip_array.first().unwrap().clone()
            } else {
                ip_array.last().unwrap().clone()
            }
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    };
    _internal_ip
}
