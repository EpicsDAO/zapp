use crate::style_print::*;
use regex::Regex;
use std::process::Command;
use std::str;

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

pub fn process_db_migrate() {
    let output = Command::new("sea-orm-cli")
        .args(&["migrate", "up"])
        .output();
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => log_error(err2),
                false => {
                    log_success("Successfully DB Migrated!");
                }
            }
        }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}

pub fn process_db_reset() {
    let output = Command::new("sea-orm-cli")
        .args(&["migrate", "reset"])
        .output();
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2);
                }
                false => {
                    log_success("Successfully DB reset!");
                }
            }
        }
        Err(err) => {
            panic!("error = {:?}", err);
        }
    }
}

pub fn process_db_refresh() {
    let output = Command::new("sea-orm-cli")
        .args(&["migrate", "refresh"])
        .output();
    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2);
                }
                false => {
                    log_success("Successfully DB refreshed!");
                }
            }
        }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}

pub fn process_db_rollback() {
    let output = Command::new("sea-orm-cli")
        .args(&["migrate", "down"])
        .output();

    match &output {
        Ok(val) => {
            let err2 = str::from_utf8(&val.stderr).unwrap();
            let rt = regex("error:");
            match rt.is_match(err2) {
                true => {
                    log_error(err2);
                }
                false => {
                    log_success("Successfully DB rollbacked!");
                }
            }
        }
        Err(err) => {
            panic!("error = {:?}", err)
        }
    }
}
