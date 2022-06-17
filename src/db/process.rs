use tokio::process::Command;
use crate::style_print::*;
use regex::Regex;
use std::str;

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

pub async fn process_db_migrate() {
    let output = Command::new("sea-orm-cli")
        .args(&[
        "migrate",
        "up"
        ])
        .output()
        .await;
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2).await
                }
                false => {
                    log_success("Successfully DB Migrated!").await
                }
              }
            }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}

pub async fn process_db_reset() {
    let output = Command::new("sea-orm-cli")
        .args(&[
        "migrate",
        "reset"
        ])
        .output()
        .await;
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2).await
                }
                false => {
                    log_success("Successfully DB reset!").await
                }
                }
            }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}

pub async fn process_db_refresh() {
    let output = Command::new("sea-orm-cli")
        .args(&[
        "migrate",
        "refresh"
        ])
        .output()
        .await;
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2).await
                }
                false => {
                    log_success("Successfully DB refreshed!").await
                }
                }
            }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}

pub async fn process_db_rollback() {
    let output = Command::new("sea-orm-cli")
        .args(&[
        "migrate",
        "down"
        ])
        .output()
        .await;
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2).await
                }
                false => {
                    log_success("Successfully DB rollbacked!").await
                }
                }
            }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}