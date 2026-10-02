use clap::{Parser, Subcommand};
use ini::Ini;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use wyag::repo::GitRepository;

#[derive(Parser)]
#[command(about = "Write Yourself a Git (in rust)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path } => {
            let _ = GitRepository::repo_create(path);
        }
    }
    Ok(())
}
