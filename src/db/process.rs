use tokio::process::Command;
use crate::style_print::*;
use regex::Regex;
use std::str;
use spinners::{Spinner, Spinners};

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

pub async fn process_db_migrate() {
    let mut sp = Spinner::new(Spinners::Aesthetic, "Migrating..".into());
    let output = Command::new("sea-orm-cli")
        .args(&[
        "migrate",
        "up"
        ])
        .output()
        .await;
    match &output {
        Ok(val) => {
            let err = str::from_utf8(&val.stdout).unwrap();
            let rt = regex("Running");
            match rt.is_match(err) {
                true => {
                    sp.stop();
                    log_success("Successfully DB Migrated!").await;
                }
                false => {
                    sp.stop();
                    panic!("{:?}", err)
                }
              }
            }
        Err(err) => {
            sp.stop();
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
            let err = str::from_utf8(&val.stdout).unwrap();
            let rt = regex("Running");
            match rt.is_match(err) {
                true => {
                    log_success("Successfully DB reset!").await;
                }
                false => {
                    panic!("{:?}", err)
                }
              }
            }
        Err(err) => panic!("error = {:?}", err)
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
            let err = str::from_utf8(&val.stdout).unwrap();
            let rt = regex("Running");
            match rt.is_match(err) {
                true => {
                    log_success("Successfully DB refreshed!").await;
                }
                false => {
                    panic!("{:?}", err)
                }
              }
            }
        Err(err) => panic!("error = {:?}", err)
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
            let err = str::from_utf8(&val.stdout).unwrap();
            let rt = regex("Running");
            match rt.is_match(err) {
                true => {
                    log_success("Successfully DB rollbacked!").await;
                }
                false => {
                    panic!("{:?}", err)
                }
              }
            }
        Err(err) => panic!("error = {:?}", err)
        }
}