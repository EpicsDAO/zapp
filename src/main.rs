use clap::Parser;
use zapp::cli::{
    Cli, Commands, ComputeCommands, DockerCommands, GcpConfig, GhCommands, IamCommands,
    InitCommands, RunCommands, SqlCommands
};
use zapp::compute::*;
use zapp::style_print::*;
use zapp::docker::*;
use zapp::gh::*;
use zapp::iam::*;
use zapp::init::*;
use zapp::run::*;
use zapp::sql::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let file_name = "gcp_config.json";
    let file_exist = Path::new(file_name).exists();
    if file_exist == false {
        process_init_gcp_config().await;
    }
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    match cli.command {
        Commands::Iam(iam) => {
            let iam_cmd = iam.command.unwrap_or(IamCommands::Help);
            match iam_cmd {
                IamCommands::Setup => {
                    process_create_service_account(
                        gcp.project_id.as_str(),
                        gcp.service_name.as_str(),
                    )
                    .await;
                    process_create_service_account_key(
                        gcp.project_id.as_str(),
                        gcp.service_name.as_str(),
                    )
                    .await;
                    process_add_roles(gcp.project_id.as_str(), gcp.service_name.as_str()).await;
                    process_enable_permissions(gcp.project_id.as_str()).await;
                    let log = "Your IAM is all set!";
                    log_success(log).await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu iam --help";
                    log_error(log).await;
                }
            }
        }
        Commands::Run(run) => {
            let run_cmd = run.command.unwrap_or(RunCommands::Help);
            match run_cmd {
                RunCommands::Build => {
                    process_gcloud_build(&gcp.project_id, &gcp.service_name).await;
                }
                RunCommands::Deploy => {
                    process_deploy(&gcp.project_id, &gcp.service_name).await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu run --help";
                    log_error(log).await;
                }
            }
        }
        Commands::Gh(gh) => {
            let gh_cmd = gh.command.unwrap_or(GhCommands::Help);
            match gh_cmd {
                GhCommands::AddEnv => {
                    process_setup_secret().await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu gh --help";
                    log_error(log).await;
                }
            }
        }
        Commands::Init(init) => {
            let init_cmd = init.command.unwrap_or(InitCommands::Help);
            match init_cmd {
                InitCommands::Config => {
                    process_init_gcp_config().await;
                }
                InitCommands::GhActions => {
                    build_api_workflow().await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu init --help";
                    log_error(log).await;
                }
            }
        }
        Commands::Compute(compute) => {
            let compute_cmd = compute.command.unwrap_or(ComputeCommands::Help);
            match compute_cmd {
                ComputeCommands::CreateNat => {
                    process_create_network(&gcp.project_id, &gcp.service_name).await;
                    process_create_firewall_tcp(&gcp.project_id, &gcp.service_name).await;
                    process_create_firewall_ssh(&gcp.project_id, &gcp.service_name).await;
                    process_create_subnet(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                    process_create_connector(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                    process_create_router(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                    process_create_external_ip(&gcp.project_id, &gcp.service_name, &gcp.region)
                        .await;
                    process_create_nat(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                }
                ComputeCommands::Setup => {
                    setup_deployment(gcp).await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu compute --help";
                    log_error(log).await;
                }
            }
        }
        Commands::Docker(docker) => {
            let docker_cmd = docker.command.unwrap_or(DockerCommands::Help);
            match docker_cmd {
                DockerCommands::Psql => {
                    process_docker_psql().await;
                }
                DockerCommands::Build => {
                    process_docker_build(&gcp.project_id, &gcp.service_name).await;
                }
                DockerCommands::Push => {
                    process_docker_push(&gcp.project_id, &gcp.service_name).await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu docker --help";
                    log_error(log).await;
                }
            }
        }
        Commands::Sql(sql) => {
            let sql_cmd = sql.command.unwrap_or(SqlCommands::Help);
            match sql_cmd {
                SqlCommands::Create => {
                    process_create_sql(&gcp.project_id, &gcp.service_name, &gcp.region).await;
                }
                SqlCommands::SetPrivateIp => {
                    process_create_ip_range(&gcp.project_id, &gcp.service_name).await;
                    process_connect_vpc_connector(&gcp.project_id, &gcp.service_name).await;
                    process_assign_network(&gcp.project_id, &gcp.service_name).await;
                }
                _ => {
                    let log = "To see example;\n\n $gcu sql --help";
                    log_error(log).await;
                }
            }
        }
    }
}

pub async fn setup_deployment(gcp: GcpConfig) {
    // 1. Create IAM
    process_create_service_account(gcp.project_id.as_str(), gcp.service_name.as_str()).await;
    process_create_service_account_key(gcp.project_id.as_str(), gcp.service_name.as_str()).await;
    process_add_roles(gcp.project_id.as_str(), gcp.service_name.as_str()).await;
    process_enable_permissions(gcp.project_id.as_str()).await;
    let log = "Your IAM is all set!";
    log_success(log).await;
    // 2. Create NAT
    process_create_network(&gcp.project_id, &gcp.service_name).await;
    process_create_firewall_tcp(&gcp.project_id, &gcp.service_name).await;
    process_create_firewall_ssh(&gcp.project_id, &gcp.service_name).await;
    process_create_subnet(&gcp.project_id, &gcp.service_name, &gcp.region).await;
    process_create_connector(&gcp.project_id, &gcp.service_name, &gcp.region).await;
    process_create_router(&gcp.project_id, &gcp.service_name, &gcp.region).await;
    process_create_external_ip(&gcp.project_id, &gcp.service_name, &gcp.region).await;
    process_create_nat(&gcp.project_id, &gcp.service_name, &gcp.region).await;
    // 3. Create Cloud SQL
    process_create_sql(&gcp.project_id, &gcp.service_name, &gcp.region).await;
    // 4. Create Cloud SQL Private Network
    process_create_ip_range(&gcp.project_id, &gcp.service_name).await;
    process_connect_vpc_connector(&gcp.project_id, &gcp.service_name).await;
    process_assign_network(&gcp.project_id, &gcp.service_name).await;
    // 5. Create Github Actions Workflow
    build_api_workflow().await;
}