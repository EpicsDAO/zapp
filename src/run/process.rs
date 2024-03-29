use std::process::Command;

pub fn process_gcloud_build(project_id: &str, service_name: &str, gcr_region: &str) {
    let gcr_url = String::from(gcr_region) + "/" + project_id + "/" + service_name;
    let output = Command::new("gcloud")
        .args(&[
            "builds",
            "submit",
            "--tag",
            &gcr_url,
            "--timeout=80000",
            "--project",
            project_id,
        ])
        .output();

    println!("output = {:?}", output);
}

pub fn process_deploy(project_id: &str, service_name: &str, gcr_region: &str) {
    let img_url = String::from(gcr_region) + "/" + project_id + "/" + service_name;
    println!("{}", &img_url);
    let output = Command::new("gcloud")
        .args(&[
            "run",
            "deploy",
            "--image",
            &img_url,
            "--project",
            project_id,
        ])
        .output();

    println!("output = {:?}", output);
}
