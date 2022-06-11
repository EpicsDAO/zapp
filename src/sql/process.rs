use tokio::process::Command;
use crate::style_print::*;
use console::style;
use std::io;
use regex::Regex;
use std::str;
use std::fs;
use std::io::Write;
use crate::gh::process_setup_secret;

#[derive(Debug)]
pub struct EnvProduction {
  pub database_url: String,
  pub zapp_gcloudsql_instance: String,
  pub zapp_gcp_project_id: String,
  pub zapp_service_name: String,
  pub zapp_gcp_region: String
}

fn regex(re_str: &str) -> Regex {
  Regex::new(re_str).unwrap()
}

pub async fn process_create_sql(project_id: &str, service_name: &str, region: &str) {
  log_input("Please input your DB Root Password:").await;
  let mut db_password = String::new();
  io::stdin()
    .read_line(&mut db_password)
    .expect("Failed to read line");
  let db_password: String = db_password
    .trim()
    .parse()
    .expect("Please input DB Root Password:");
  let zone = String::from(region) + "-b";
  println!(
    "⏰ {}",
    style("Creating Cloud SQL ...\nThis process takes 5 to 10 min.").white().bold()
  );
  let instance_name = String::from(service_name) + "-db";
  let internal_ip = get_instance_ip(project_id, service_name, 1).await;
  let database_url = String::from("DATABASE_URL='postgres://postgres:") + &db_password + "@" + &internal_ip + ":5432/" + &instance_name + "\n";
  let zapp_gcloudsql_instance = String::from("ZAPP_GCLOUDSQL_INSTANCE=/cloudsql/") + &project_id + ":" + &region + ":" + &instance_name  + "\n";
  let zapp_gcp_project_id = String::from("ZAPP_GCP_PROJECT_ID=") + &project_id  + "\n";
  let zapp_service_name = String::from("ZAPP_SERVICE_NAME=") + &service_name  + "\n";
  let zapp_gcp_region = String::from("ZAPP_GCP_REGION=") + &region  + "\n";

  let env_production = EnvProduction {
    database_url,
    zapp_gcloudsql_instance,
    zapp_gcp_project_id,
    zapp_service_name,
    zapp_gcp_region
  };
  let filename = ".env.production";
  let mut file = fs::File::create(filename).unwrap();
  file.write_all(env_production.database_url.as_bytes()).unwrap();
  file.write_all(env_production.zapp_gcloudsql_instance.as_bytes()).unwrap();
  file.write_all(env_production.zapp_gcp_project_id.as_bytes()).unwrap();
  file.write_all(env_production.zapp_service_name.as_bytes()).unwrap();
  file.write_all(env_production.zapp_gcp_region.as_bytes()).unwrap();
  process_setup_secret().await;
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
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          println!(
              "✅ {}",
              style("Successfully created Cloud SQL!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_patch_sql(project_id: &str, service_name: &str, action: &str) {
  let instance_name = String::from(service_name) + "-db";
  let activation_policy = match action {
    "start" => {
      "ALWAYS"
    }
    "stop" => {
      "NEVER"
    }
    _ => {
      panic!("No action name!");
    }
  };
  let output = Command::new("gcloud")
    .args(&[
      "sql",
      "instances",
      "patch",
      &instance_name,
      "--activation-policy",
      activation_policy,
      "--project",
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          println!(
              "✅ {}",
              style("Successfully patched Cloud SQL!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_restart_sql(project_id: &str, service_name: &str) {
  let instance_name = String::from(service_name) + "-db";
  let output = Command::new("gcloud")
    .args(&[
      "sql",
      "instances",
      "restart",
      &instance_name,
      "--project",
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          log_success("Successfully restart Cloud SQL!").await
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_create_ip_range(project_id: &str, service_name: &str) {
  println!(
    "⏰ {}",
    style("Creating IP range ...\nThis process takes 5 to 10 min.").white().bold()
  );
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
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          println!(
              "✅ {}",
              style("Successfully created IP range!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_connect_vpc_connector(project_id: &str, service_name: &str) {
  println!(
    "⏰ {}",
    style("Connecting to VPC Connector ...\nThis process takes 5 to 10 min.").white().bold()
  );
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
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          println!(
              "✅ {}",
              style("Successfully connected to VPC!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_assign_network(project_id: &str, service_name: &str) {
  println!(
    "⏰ {}",
    style("Assign network ...\nThis process takes 5 to 10 min.").white().bold()
  );
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
      project_id
    ])
    .output()
    .await;
  match &output {
    Ok(val) => {
      let err = str::from_utf8(&val.stderr);
      let rt = regex("ERROR:");
      match rt.is_match(err.unwrap()) {
        true => {
            panic!("{:?}", err.unwrap())
        }
        false => {
          println!(
              "✅ {}",
              style("Successfully setup your database!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}


pub async fn get_instance_ip(project_id: &str, service_name: &str, ip_type: usize) -> String {
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
      "value(ipAddresses.ipAddress)"
    ])
    .output()
    .await;
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
    },
    Err(err) => {
      panic!("{:?}", err)
    }
  };
  _internal_ip
}


// async fn region_to_timezone(region: &str) -> &str {
//   let asia = regex("asia");
//   let eu = regex("europe");
//   let zone = if asia.is_match(region)  {
//     "Asia/Tokyo"
//   } else if eu.is_match(region) {
//     "Europe/Amsterdam"
//   } else {
//     "America/Los_Angeles"
//   };
//   zone
// }