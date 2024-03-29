use crate::style_print::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

pub fn process_setup_secret() {
    let filename = ".env.production";
    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        let array = line.split('=').fold(Vec::new(), |mut s, i| {
            s.push(i.to_string());
            s
        });
        let key = &array[0];
        let value = &array[1];
        process_add_env(key, value);
    }
}

pub fn process_add_env(key: &str, value: &str) {
    let output = Command::new("gh")
        .args(&["secret", "set", key, "-b", value])
        .output();

    match &output {
        Ok(_v) => log_success(&format!("Successfully added: {}", key)),
        Err(err) => panic!("{}", err),
    }
}
