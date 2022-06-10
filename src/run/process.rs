use tokio::process::Command;

pub async fn process_gcloud_build(project_id: &str, service_name: &str) {
  let gcr_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  let output = Command::new("gcloud")
    .args(
      &[
        "builds",
        "submit",
        "--tag",
        &gcr_url,
        "--timeout=80000",
        "--project",
        project_id
        ])
    .output()
    .await;
  println!("output = {:?}", output);
}

pub async fn process_deploy(project_id: &str, service_name: &str) {
  let img_url = String::from("eu.gcr.io/") + project_id + "/" + service_name;
  println!("{}", &img_url);
  let output = Command::new("gcloud")
    .args(
      &[
        "run",
        "deploy",
        "--image",
        &img_url,
        "--project",
        project_id
        ])
    .output()
    .await;
  println!("output = {:?}", output);
}
