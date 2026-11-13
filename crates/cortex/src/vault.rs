use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use include_dir::{Dir, include_dir};

// Agent folder
static AGENT_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../vault-template/Agent");

#[derive(Debug, PartialEq)]
pub enum VaultKind {
    New,
    Existing,
}

pub fn check_path(path: &Path) -> Result<VaultKind> {
    if !path.exists() {
        return Ok(VaultKind::New);
    }

    if !path.is_dir() {
        bail!("{} is a file, not a folder", path.display());
    }
    if fs::read_dir(path)?.next().is_none() {
        return Ok(VaultKind::New);
    }
    if path.join("Agent").exists() {
        bail!(
            "{} already has an Agent folder; leaving it alone",
            path.display()
        );
    }

    if !path.join(".obsidian").is_dir() {
        bail!(
            "{} has files in it but isn't an Obsidian vault (no .obsidian folder)",
            path.display()
        );
    }

    Ok(VaultKind::Existing)
}

// Send Agent Folder into the vault and create the vault if needed
pub fn install_agent_folder(vault: &Path) -> Result<()> {
    let dest = vault.join("Agent");
    fs::create_dir_all(&dest).with_context(|| format!("couldn't create {}", dest.display()))?;
    AGENT_TEMPLATE
        .extract(&dest)
        .with_context(|| format!("couldn't copy the Agent folder into {}", vault.display()))?;
    Ok(())
}

/// ~/.config/engram/vault on every platform (the boot hook reads the same file).
pub fn config_file() -> Result<PathBuf> {
    let home = dirs::home_dir().context("couldn't find the home folder")?;
    Ok(home.join(".config").join("engram").join("vault"))
}

pub fn save_vault_path(vault: &Path) -> Result<()> {
    let file = config_file()?;
    fs::create_dir_all(file.parent().unwrap())?;
    fs::write(&file, format!("{}\n", vault.display()))?;
    Ok(())
}

/// ENGRAM_VAULT wins, then the config file. None means no vault set up yet.
pub fn find_vault() -> Result<Option<PathBuf>> {
    if let Ok(path) = std::env::var("ENGRAM_VAULT") {
        return Ok(Some(PathBuf::from(path)));
    }
    match fs::read_to_string(config_file()?) {
        Ok(text) => Ok(Some(PathBuf::from(text.trim()))),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempfile;

    use super::*;

    #[test]
    fn missing_or_empty_folder_is_new() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(
            check_path(&tmp.path().join("nope")).unwrap(),
            VaultKind::New
        );
        assert_eq!(check_path(tmp.path()).unwrap(), VaultKind::New);
    }

    #[test]
    fn folder_with_files_but_no_obsidian_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("notes.md"), "hi").unwrap();
        assert!(check_path(tmp.path()).is_err());
    }

    #[test]
    fn obsidian_vault_is_existing_unless_agent_is_there() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join(".obsidian")).unwrap();
        assert_eq!(check_path(tmp.path()).unwrap(), VaultKind::Existing);
        fs::create_dir(tmp.path().join("Agent")).unwrap();
        assert!(check_path(tmp.path()).is_err());
    }

    #[test]
    fn install_copies_the_agent_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let vault = tmp.path().join("New Vault");
        install_agent_folder(&vault).unwrap();
        assert!(vault.join("Agent/Owner Profile.md").is_file());
        assert!(vault.join("Agent/Log/Log.md").is_file());
    }

    #[test]
    fn rerunning_init_on_a_new_vault_names_the_agent_folder() {
        let tmp = tempfile::tempdir().unwrap();
        install_agent_folder(tmp.path()).unwrap();
        let err = check_path(tmp.path()).unwrap_err();
        assert!(err.to_string().contains("already has an Agent folder"));
    }
}
