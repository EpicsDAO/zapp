use crate::style_print::*;
use regex::Regex;
use std::str;
use tokio::process::Command;

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

pub async fn process_create_network(project_id: &str, service_name: &str) {
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "networks",
            "create",
            service_name,
            "--project",
            project_id,
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
                    log_success("Successfully created Network!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_firewall_tcp(project_id: &str, service_name: &str) {
    let firewall = String::from(service_name) + "-tcp";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "firewall-rules",
            "create",
            &firewall,
            "--network",
            service_name,
            "--allow",
            "tcp,udp,icmp",
            "--source-ranges",
            "10.124.0.0/28",
            "--project",
            project_id,
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
                    log_success("Successfully created Firewall TCP Rule!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_firewall_ssh(project_id: &str, service_name: &str) {
    let firewall = String::from(service_name) + "-ssh";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "firewall-rules",
            "create",
            &firewall,
            "--network",
            service_name,
            "--allow",
            "tcp:22,tcp:3389,icmp",
            "--project",
            project_id,
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
                    log_success("Successfully created Firewall SSH Rule!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_subnet(project_id: &str, service_name: &str, region: &str) {
    let subnet = String::from(service_name) + "-subnet";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "networks",
            "subnets",
            "create",
            &subnet,
            "--range",
            "10.124.0.0/28",
            "--network",
            service_name,
            "--region",
            region,
            "--project",
            project_id,
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
                    log_success("Successfully created Subnet!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_connector(project_id: &str, service_name: &str, region: &str) {
    let subnet = String::from(service_name) + "-subnet";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "networks",
            "vpc-access",
            "connectors",
            "create",
            service_name,
            "--subnet",
            &subnet,
            "--subnet-project",
            project_id,
            "--region",
            region,
            "--project",
            project_id,
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
                    log_success("Successfully created VPC Connector!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_router(project_id: &str, service_name: &str, region: &str) {
    let router = String::from(service_name) + "-router";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "routers",
            "create",
            &router,
            "--network",
            service_name,
            "--region",
            region,
            "--project",
            project_id,
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
                    log_success("Successfully created Router!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_external_ip(project_id: &str, service_name: &str, region: &str) {
    let external_ip = String::from(service_name) + "-ip";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "addresses",
            "create",
            &external_ip,
            "--region",
            region,
            "--project",
            project_id,
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
                    log_success("Successfully created External IP!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}

pub async fn process_create_nat(project_id: &str, service_name: &str, region: &str) {
    let nat = String::from(service_name) + "-nat";
    let router = String::from(service_name) + "-router";
    let nat_custom_subnet_ip_ranges = String::from(service_name) + "-subnet";
    let nat_external_ip_pool = String::from(service_name) + "-ip";
    let output = Command::new("gcloud")
        .args(&[
            "compute",
            "routers",
            "nats",
            "create",
            &nat,
            "--router",
            &router,
            "--region",
            region,
            "--nat-custom-subnet-ip-ranges",
            &nat_custom_subnet_ip_ranges,
            "--nat-external-ip-pool",
            &nat_external_ip_pool,
            "--project",
            project_id,
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
                    log_success("Successfully created Cloud NAT!").await;
                }
            }
        }
        Err(err) => println!("error = {:?}", err),
    }
}
