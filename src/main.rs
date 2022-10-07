use chrono::Local;
use clap::Parser;
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;
use zapp::cli::{
    Cli, Commands, ComputeCommands, DbCommands, DockerCommands, GcloudCommands, GcpConfig,
    GenCommands, GhCommands, IamCommands, InitCommands, RunCommands, SqlCommands,
};
use zapp::compute::*;
use zapp::db::*;
use zapp::docker::*;
use zapp::gen::*;
use zapp::gh::*;
use zapp::iam::*;
use zapp::init::*;
use zapp::run::*;
use zapp::sql::*;
use zapp::style_print::*;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { app_name } => {
            dl_zapp(&app_name);
            create_dockerfile(&app_name);
            create_env(&app_name);
            endroll(&app_name);
        }
        Commands::Gcloud(gcloud) => {
            let gcp = get_gcp();
            let gcloud_cmd = gcloud.command.unwrap_or(GcloudCommands::Help);
            match gcloud_cmd {
                GcloudCommands::Setup => {
                    setup_deployment(gcp);
                }
                _ => {
                    let log = "To see example;\n\n $zapp gcloud --help";
                    log_error(log);
                }
            }
        }
        Commands::Iam(iam) => {
            let gcp = get_gcp();
            let iam_cmd = iam.command.unwrap_or(IamCommands::Help);
            match iam_cmd {
                IamCommands::Setup => {
                    process_create_service_account(
                        gcp.project_id.as_str(),
                        gcp.service_name.as_str(),
                    );

                    process_create_service_account_key(
                        gcp.project_id.as_str(),
                        gcp.service_name.as_str(),
                    );

                    process_add_roles(gcp.project_id.as_str(), gcp.service_name.as_str());
                    process_enable_permissions(gcp.project_id.as_str());
                    set_keyfile_to_gh_secret();
                    let log = "Your IAM is all set!";
                    log_success(log);
                }
                _ => {
                    let log = "To see example;\n\n $zapp iam --help";
                    log_error(log);
                }
            }
        }
        Commands::Run(run) => {
            let gcp = get_gcp();
            let run_cmd = run.command.unwrap_or(RunCommands::Help);
            match run_cmd {
                RunCommands::Build => {
                    process_gcloud_build(&gcp.project_id, &gcp.service_name, &gcp.gcr_region());
                }
                RunCommands::Deploy => {
                    process_deploy(&gcp.project_id, &gcp.service_name, &gcp.gcr_region());
                }
                _ => {
                    let log = "To see example;\n\n $zapp run --help";
                    log_error(log);
                }
            }
        }
        Commands::Gh(gh) => {
            let gh_cmd = gh.command.unwrap_or(GhCommands::Help);
            match gh_cmd {
                GhCommands::AddEnv => {
                    process_setup_secret();
                }
                _ => {
                    let log = "To see example;\n\n $zapp gh --help";
                    log_error(log);
                }
            }
        }
        Commands::Init(init) => {
            let init_cmd = init.command.unwrap_or(InitCommands::Help);
            match init_cmd {
                InitCommands::Config => {
                    process_init_gcp_config();
                }
                InitCommands::GhActions => {
                    let gcp = get_gcp();
                    build_api_workflow(gcp.gcr_region());
                }
                _ => {
                    let log = "To see example;\n\n $zapp init --help";
                    log_error(log);
                }
            }
        }
        Commands::Compute(compute) => {
            let gcp = get_gcp();
            let compute_cmd = compute.command.unwrap_or(ComputeCommands::Help);
            match compute_cmd {
                ComputeCommands::CreateNat => {
                    process_create_network(&gcp.project_id, &gcp.service_name);
                    process_create_firewall_tcp(&gcp.project_id, &gcp.service_name);
                    process_create_firewall_ssh(&gcp.project_id, &gcp.service_name);
                    process_create_subnet(&gcp.project_id, &gcp.service_name, &gcp.region);
                    process_create_connector(&gcp.project_id, &gcp.service_name, &gcp.region);
                    process_create_router(&gcp.project_id, &gcp.service_name, &gcp.region);
                    process_create_external_ip(&gcp.project_id, &gcp.service_name, &gcp.region);
                    process_create_nat(&gcp.project_id, &gcp.service_name, &gcp.region);
                }
                _ => {
                    let log = "To see example;\n\n $zapp compute --help";
                    log_error(log);
                }
            }
        }
        Commands::Docker(docker) => {
            let docker_cmd = docker.command.unwrap_or(DockerCommands::Help);
            match docker_cmd {
                DockerCommands::Psql => {
                    process_psql_docker();
                }
                DockerCommands::Build => {
                    let gcp = get_gcp();
                    process_docker_build(&gcp.project_id, &gcp.service_name, &gcp.gcr_region());
                }
                DockerCommands::Restart => {
                    process_docker_restart();
                }
                DockerCommands::Push => {
                    let gcp = get_gcp();
                    process_docker_push(&gcp.project_id, &gcp.service_name, &gcp.gcr_region());
                }
                _ => {
                    let log = "To see example;\n\n $zapp docker --help";
                    log_error(log);
                }
            }
        }
        Commands::Sql(sql) => {
            let gcp = get_gcp();
            let sql_cmd = sql.command.unwrap_or(SqlCommands::Help);
            match sql_cmd {
                SqlCommands::Create => {
                    process_create_sql(
                        &gcp.project_id,
                        &gcp.service_name,
                        &gcp.region,
                        &gcp.network,
                    );
                }
                SqlCommands::Patch { action } => {
                    process_patch_sql(&gcp.project_id, &gcp.service_name, &action);
                }
                SqlCommands::Restart => {
                    process_restart_sql(&gcp.project_id, &gcp.service_name);
                }
                SqlCommands::SetPrivateIp => {
                    process_create_ip_range(&gcp.project_id, &gcp.service_name);
                    process_connect_vpc_connector(&gcp.project_id, &gcp.service_name);
                    process_assign_network(&gcp.project_id, &gcp.service_name);
                }
                _ => {
                    let log = "To see example;\n\n $zapp sql --help";
                    log_error(log);
                }
            }
        }
        Commands::Gen(gen) => {
            let gen_cmd = gen.command.unwrap_or(GenCommands::Help);
            match gen_cmd {
                GenCommands::Model { model, path } => {
                    let gen_path_buf = path.unwrap_or_else(|| current_dir().unwrap());
                    let gen_path = gen_path_buf.as_path();
                    let date = Local::now();
                    handle_gen(&model, date.naive_local(), gen_path);
                }
                _ => {
                    let log = "To see example;\n\n $zapp run --help";
                    log_error(log);
                }
            }
        }
        Commands::Db(db) => {
            let db_cmd = db.command.unwrap_or(DbCommands::Help);
            match db_cmd {
                DbCommands::Migrate => {
                    process_db_migrate();
                }
                DbCommands::Reset => {
                    process_docker_restart();
                    process_db_migrate();
                }
                DbCommands::Refresh => {
                    process_db_refresh();
                }
                DbCommands::Rollback => {
                    process_db_rollback();
                }
                _ => {
                    let log = "To see example;\n\n $zapp db --help";
                    log_error(log);
                }
            }
        }
    }
}

pub fn get_gcp() -> GcpConfig {
    let file_name = "gcp_config.json";
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let gcp: GcpConfig = serde_json::from_reader(reader).unwrap();
    gcp
}

pub fn setup_deployment(gcp: GcpConfig) {
    // 1. Create IAM
    process_create_service_account(&gcp.project_id, &gcp.service_name);
    process_create_service_account_key(&gcp.project_id, &gcp.service_name);
    process_add_roles(&gcp.project_id, &gcp.service_name);
    process_enable_permissions(&gcp.project_id);
    let log = "Your IAM is all set!";
    log_success(log);
    // 2. Create NAT
    process_create_network(&gcp.project_id, &gcp.service_name);
    process_create_firewall_tcp(&gcp.project_id, &gcp.service_name);
    process_create_firewall_ssh(&gcp.project_id, &gcp.service_name);
    process_create_subnet(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_connector(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_router(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_external_ip(&gcp.project_id, &gcp.service_name, &gcp.region);
    process_create_nat(&gcp.project_id, &gcp.service_name, &gcp.region);
    // 3. Create Cloud SQL
    process_create_sql(
        &gcp.project_id,
        &gcp.service_name,
        &gcp.region,
        &gcp.network,
    );
    // 4. Create Cloud SQL Private Network
    process_create_ip_range(&gcp.project_id, &gcp.service_name);
    process_connect_vpc_connector(&gcp.project_id, &gcp.service_name);
    process_assign_network(&gcp.project_id, &gcp.service_name);
    // 5. Create Github Actions Workflow
    build_api_workflow(&gcp.gcr_region());
}
