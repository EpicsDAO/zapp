use tokio::process::Command;
use std::str;
use console::style;
use regex::Regex;

fn regex(re_str: &str) -> Regex {
  Regex::new(re_str).unwrap()
}

pub async fn process_create_service_account(project_id: &str, service_name: &str) {
  let description = String::from("--description='") + service_name + " Service Account'";
  let display_name = String::from("--display-name=") + service_name;
  let output = Command::new("gcloud")
    .args(&[
      "iam",
      "service-accounts",
      "create",
      service_name,
      description.as_str(),
      display_name.as_str(),
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
              style("Successfully created service account!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_create_service_account_key(project_id: &str, service_name: &str) {
  let service_account = String::from(service_name) + "@" + project_id + ".iam.gserviceaccount.com";
  let output = Command::new("gcloud")
    .args(&[
      "iam",
      "service-accounts",
      "keys",
      "create",
      "./keyfile.json",
      "--iam-account",
      service_account.as_str(),
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
              style("Successfully exported keyfile!").white().bold()
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_add_roles(project_id: &str, service_name: &str) {
  let roles = [
    "roles/cloudsql.editor",
    "roles/containerregistry.ServiceAgent",
    "roles/pubsub.editor",
    "roles/datastore.user",
    "roles/iam.serviceAccountUser",
    "roles/run.admin",
    "roles/storage.admin",
    "roles/storage.objectAdmin",
    "roles/cloudscheduler.admin",
    "roles/appengine.appCreator",
    "roles/logging.admin",
    "roles/cloudtranslate.admin",
  ];
  for role in roles {
    process_add_service_account_role(project_id, service_name, role).await;
  }
}

pub async fn process_add_service_account_role(
  project_id: &str,
  service_name: &str,
  role_arg: &str,
) {
  let member = String::from("--member=serviceAccount:")
    + service_name
    + "@"
    + project_id
    + ".iam.gserviceaccount.com";
  let role = String::from("--role=") + role_arg;
  let output = Command::new("gcloud")
    .args(&[
      "projects",
      "add-iam-policy-binding",
      project_id,
      member.as_str(),
      role.as_str(),
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
              "✅ {} {}",
              style("Successfully added role:").white().bold(),
              role_arg
          );
        }
      }
    },
    Err(err) => println!("error = {:?}", err)
  }
}

pub async fn process_enable_permissions(project_id: &str) {
  let service_urls = [
    "compute.googleapis.com",
    "iam.googleapis.com",
    "dns.googleapis.com",
    "sqladmin.googleapis.com",
    "sql-component.googleapis.com",
    "servicenetworking.googleapis.com",
    "containerregistry.googleapis.com",
    "run.googleapis.com",
    "vpcaccess.googleapis.com",
    "cloudscheduler.googleapis.com",
    "cloudresourcemanager.googleapis.com",
    "translate.googleapis.com",
    "firestore.googleapis.com",
    "cloudfunctions.googleapis.com",
    "cloudbuild.googleapis.com",
    "spanner.googleapis.com",
  ];
  for service_name in service_urls {
    let output = Command::new("gcloud")
      .args(
        &[
          "services",
          "enable",
          service_name,
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
                "✅ {} {}",
                style("Enabled API:").white().bold(),
                service_name
            );
          }
        }
      },
      Err(err) => println!("error = {:?}", err)
    }
  }
}
