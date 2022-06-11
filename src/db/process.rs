use tokio::process::Command;

pub async fn process_db_migrate() {
    let output = Command::new("sea-orm-cli")
        .args(&[
        "migrate",
        "up"
        ])
        .output()
        .await;
    println!("{:?}", &output.unwrap());
}
