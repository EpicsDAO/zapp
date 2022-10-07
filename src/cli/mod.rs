use clap::{Args, Parser, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
    pub project_id: String,
    pub service_name: String,
    pub region: String,
    pub network: String,
}

fn regex(re_str: &str) -> Regex {
    Regex::new(re_str).unwrap()
}

impl GcpConfig {
    pub fn gcr_region(&self) -> &str {
        let asia = regex("asia");
        let eu = regex("europe");
        return if asia.is_match(&self.region) {
            "asia.gcr.io"
        } else if eu.is_match(&self.region) {
            "eu.gcr.io"
        } else {
            "gcr.io"
        };
    }
}

#[derive(Parser)]
#[clap(name = "zapp")]
#[clap(about = "Rust Serverless Framework")]
#[clap(author = "EpicsDAO", version, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Iam(Iam),
    Run(Run),
    Gh(Gh),
    Init(Init),
    Compute(Compute),
    Docker(Docker),
    Sql(Sql),
    Gen(Gen),
    Db(Db),
    New { app_name: String },
    Gcloud(Gcloud),
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Iam {
    #[clap(subcommand)]
    pub command: Option<IamCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Compute {
    #[clap(subcommand)]
    pub command: Option<ComputeCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Init {
    #[clap(subcommand)]
    pub command: Option<InitCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Gh {
    #[clap(subcommand)]
    pub command: Option<GhCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Run {
    #[clap(subcommand)]
    pub command: Option<RunCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Docker {
    #[clap(subcommand)]
    pub command: Option<DockerCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Sql {
    #[clap(subcommand)]
    pub command: Option<SqlCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Gen {
    #[clap(subcommand)]
    pub command: Option<GenCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Db {
    #[clap(subcommand)]
    pub command: Option<DbCommands>,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Gcloud {
    #[clap(subcommand)]
    pub command: Option<GcloudCommands>,
}

#[derive(Debug, Subcommand)]
pub enum IamCommands {
    Setup,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum ComputeCommands {
    CreateNat,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum InitCommands {
    Config,
    GhActions,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum GhCommands {
    AddEnv,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum RunCommands {
    Build,
    Deploy,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum DockerCommands {
    Psql,
    Build,
    Restart,
    Push,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum SqlCommands {
    Create,
    Patch { action: String },
    Restart,
    SetPrivateIp,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum GenCommands {
    Model {
        model: String,
        #[clap(parse(from_os_str), short, long)]
        path: Option<PathBuf>,
    },
    Help,
}

#[derive(Debug, Subcommand)]
pub enum DbCommands {
    Migrate,
    Reset,
    Refresh,
    Rollback,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum GcloudCommands {
    Setup,
    Help,
}
