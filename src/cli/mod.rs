use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GcpConfig {
    pub project_id: String,
    pub service_name: String,
    pub region: String,
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
    New {
        app_name: String
    }
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

#[derive(Debug, Subcommand)]
pub enum IamCommands {
    Setup,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum ComputeCommands {
    CreateNat,
    Setup,
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
    Push,
    Help,
}

#[derive(Debug, Subcommand)]
pub enum SqlCommands {
    Create,
    Patch {
        action: String
    },
    Restart,
    SetPrivateIp,
    Help
}
