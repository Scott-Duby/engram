use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use cortex::boot::{self, Part};
use cortex::vault::{self, VaultKind};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

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
    Boot {
        part: BootPart,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum BootPart {
    Index,
    Profile,
    State,
    Today,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { path } => init(&path)?,
        Command::Vault => match vault::find_vault()? {
            Some(path) => println!("{}", path.display()),
            None => bail!("no vault set up yet; run `brain init <PATH>` first"),
        },
        Command::Boot { part } => boot_part(part),
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

fn boot_part(part: BootPart) {
    let reason = match vault::find_vault() {
        Ok(Some(dir)) => {
            let text = match part {
                BootPart::Index => boot::build_part(Part::Index, &dir),
                BootPart::Profile => boot::build_part(Part::Profile, &dir),
                BootPart::State => boot::build_part(Part::State, &dir),
                BootPart::Today => match jiff::ZonedDateTime::try_from(SystemTime::now()) {
                    Ok(now) => boot::build_today(&dir, &now.date().to_string()),
                    // Clock outside jiff's range; the hook must still exit 0
                    Err(e) => format!(
                        "Engram boot context (part 4 of 4: today)\n\nCouldn't read today's date: {e}. Read the latest log in Agent/Log directly.\n",
                    ),
                },
            };
            print!("{text}");
            return;
        }
        Ok(None) => "No vault set up yet".to_string(),
        Err(e) => format!("The vault couldn't be loaded: {e:#}"),
    };
    if matches!(part, BootPart::Index) {
        print!("{}", boot::missing_vault(&reason));
    }
}
