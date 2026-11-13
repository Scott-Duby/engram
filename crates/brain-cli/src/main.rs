use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use cortex::vault::{self, VaultKind};
use std::path::{Path, PathBuf};

/// Sets up and manages Engram vault
#[derive(Parser)]
#[command(name = "brain", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Set up a vault at PATH (new folder or existing Obsidian vault)
    Init {
        /// Folder for the vault
        path: PathBuf,
    },
    /// Print the vault in use
    Vault,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { path } => init(&path)?,
        Command::Vault => match vault::find_vault()? {
            Some(path) => println!("{}", path.display()),
            None => bail!("no vault set up yet; run `brain init <PATH>` first"),
        },
    }
    Ok(())
}

fn init(path: &Path) -> Result<()> {
    let dir = std::path::absolute(path)
        .with_context(|| format!("couldn't resolve {}", path.display()))?;
    let kind = vault::check_path(&dir)?;
    vault::install_agent_folder(&dir)?;
    vault::save_vault_path(&dir)?;

    match kind {
        VaultKind::New => println!("Created a new vault at {}", dir.display()),
        VaultKind::Existing => println!("Added the Agent folder to {}", dir.display()),
    }
    Ok(())
}
