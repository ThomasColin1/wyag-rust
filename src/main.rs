use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
use wyag::object::{self};
use wyag::repo::{GitRepository, WyagError};

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
    CatFile {
        #[arg(long, default_value = "blob")]
        object_type: String,
        #[arg(short, long)]
        object: String,
    },
    HashObject {
        #[arg(short)]
        write: bool,
        #[arg(short, default_value = "blob")]
        object_type: String,
        #[arg()]
        file: PathBuf,
    },
}

pub fn object_find(_repo: &GitRepository, name: &str, _fmt: &str, _follow: bool) -> String {
    name.to_string()
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path } => {
            let _ = GitRepository::repo_create(path);
        }
        Commands::CatFile {
            object_type,
            object,
        } => {
            if object_type != "blob" {
                todo!();
            }
            let mut cmd = Cli::command();

            let repo_find = &GitRepository::repo_find(Path::new("."), true);
            let repo = match repo_find {
                Ok(Some(git_repo)) => git_repo,
                Err(err) => cmd.error(ErrorKind::Format, err).exit(),
                Ok(None) => cmd.error(ErrorKind::Format, WyagError::TODOERROR()).exit(),
            };

            let obj = object::object_read(repo, &object_find(repo, object, "", true))
                .unwrap_or_else(|err| cmd.error(ErrorKind::Format, err).exit());

            let serialized = String::from_utf8(obj.serialize())
                .unwrap_or_else(|err| cmd.error(ErrorKind::Format, err).exit());

            println!("{}", serialized);
        }
        Commands::HashObject {
            write,
            object_type,
            file,
        } => {
            let mut cmd = Cli::command();
            let repo;
            let repo_find;

            if *write {
                repo_find = GitRepository::repo_find(Path::new("."), true);
                repo = match &repo_find {
                    Ok(Some(git_repo)) => Some(git_repo),
                    Err(err) => cmd.error(ErrorKind::Format, err).exit(),
                    Ok(None) => None,
                };
            } else {
                repo = None;
            }

            let mut file_open = File::open(file)?;
            let mut data: Vec<u8> = Vec::new();
            file_open.read_to_end(&mut data)?;

            let obj: object::Object = match object_type.as_str() {
                "commit" => object::Object::Commit(object::GitCommit { data }),
                "tree" => object::Object::Tree(object::GitTree { data }),
                "tag" => object::Object::Tag(object::GitTag { data }),
                "blob" => object::Object::Blob(object::GitBlob { data }),
                _ => cmd.error(ErrorKind::Format, WyagError::TODOERROR()).exit(),
            };
            let res = object::object_write(obj, repo)
                .unwrap_or_else(|err| cmd.error(ErrorKind::Format, err).exit());

            println!("{}", res);
        }
    }
    Ok(())
}
