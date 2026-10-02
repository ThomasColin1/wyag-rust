use clap::{Parser, Subcommand};
use ini::Ini;
use std::fs;
use std::io::{self};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use thiserror::Error;

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

pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
    pub conf: Ini,
}

#[derive(Error, Debug)]
pub enum WyagError {
    #[error("Not a git repository : {0}")]
    NotARepository(PathBuf),
    #[error("Element not found : {0}")]
    NotFound(PathBuf),
    #[error("Configuration file missing in repository")]
    ConfigMissing,
    #[error("Error while parsing configuration file")]
    ConfigParsingError,
    #[error("Unsupported repositoryformatversion : {0}")]
    UnsupportedVersion(i32),
    #[error("Path is not a directory : {0}")]
    NotADirectory(PathBuf),
    #[error("Git directory is not empty : {0}")]
    DirectoryNotEmpty(PathBuf),
    #[error("I/O error : {0}")]
    Io(#[from] io::Error),
    #[error("Failed to parse configuration : {0}")]
    Ini(#[from] ini::Error),
    #[error("Invalid integer parse : {0}")]
    ParseInt(#[from] ParseIntError),
}

impl GitRepository {
    // Initializes the object (force=true for new repo)
    pub fn init(path: &Path, force: bool) -> Result<Self, WyagError> {
        let worktree = path.to_path_buf();
        let gitdir = Path::new(path).join(".git");
        let mut conf = Ini::new();

        if !force && !gitdir.is_dir() {
            return Err(WyagError::NotARepository(worktree));
        }

        let config_path = gitdir.join("config");

        if !force && !config_path.exists() {
            return Err(WyagError::ConfigMissing);
        } else if !force {
            conf = Ini::load_from_file(config_path)?;
            let section = conf
                .section(Some("core"))
                .ok_or(WyagError::ConfigParsingError)?;
            let version_str = section
                .get("repositoryformatversion")
                .ok_or(WyagError::ConfigParsingError)?;
            let version: i32 = version_str.parse()?;
            if version != 0 {
                return Err(WyagError::UnsupportedVersion(version));
            }
        }
        Ok(GitRepository {
            worktree,
            gitdir,
            conf,
        })
    }

    // Gives the path of a directory (creates it if mkdir=true)
    pub fn repo_dir<P: AsRef<Path>>(&self, path: P, mkdir: bool) -> Result<PathBuf, WyagError> {
        let full_path = self.gitdir.join(path);

        if full_path.exists() {
            if full_path.is_dir() {
                Ok(full_path)
            } else {
                Err(WyagError::NotADirectory(full_path))
            }
        } else if mkdir {
            fs::create_dir_all(&full_path)?;
            Ok(full_path)
        } else {
            Err(WyagError::NotFound(full_path))
        }
    }

    // Gives the path of a file (creates the parent path if mkdir=true)
    pub fn repo_file<P: AsRef<Path>>(&self, path: P, mkdir: bool) -> Result<PathBuf, WyagError> {
        if let Some(parent) = path.as_ref().parent() {
            self.repo_dir(parent, mkdir)?;
        }

        Ok(self.gitdir.join(path))
    }

    // Initializes the default config file
    pub fn repo_default_config() -> Ini {
        let mut conf = Ini::new();
        conf.with_section(Some("core"))
            .set("repositoryformatversion", "0")
            .set("filemode", "false")
            .set("bare", "false");

        conf
    }

    // Creates the repo (= git init)
    pub fn repo_create(path: &Path) -> Result<GitRepository, WyagError> {
        let repo = GitRepository::init(path, true)?;

        if repo.worktree.exists() {
            if !repo.worktree.is_dir() {
                return Err(WyagError::NotADirectory(repo.worktree));
            }
            if repo.gitdir.exists() {
                let is_not_empty = fs::read_dir(&repo.gitdir)? // Creates iterator
                    .next() // First element
                    .is_some(); // Does it exist ?

                if is_not_empty {
                    return Err(WyagError::DirectoryNotEmpty(repo.gitdir));
                }
            }
        } else {
            fs::create_dir_all(&repo.worktree)?;
        }

        repo.repo_dir("branches", true)?;
        repo.repo_dir("objects", true)?;
        repo.repo_dir("refs/tags", true)?;
        repo.repo_dir("refs/heads", true)?;

        let desc_path = repo.repo_file("description", true)?;
        let head_path = repo.repo_file("HEAD", true)?;
        let config_path = repo.repo_file("config", true)?;

        fs::write(
            &desc_path,
            "Unnamed repository; edit this file \
            'description' to name the repository.\n",
        )?;

        fs::write(&head_path, "ref: refs/heads/master\n")?;

        Self::repo_default_config().write_to_file(&config_path)?;

        Ok(repo)
    }
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
