use std::env;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

const MARKETPLACE: &str = "Scott-Duby/engram";
const PLUGIN: &str = "engram@engram";

/// Adds the Engram marketplace to Claude Code and installs the plugin; safe to rerun
pub fn claude_code() -> Result<()> {
    run_claude(&["plugin", "marketplace", "add", MARKETPLACE])?;
    run_claude(&["plugin", "install", PLUGIN, "--scope", "user"])
}

// shortcut: finds `claude` by plain name, which misses an npm `claude.cmd` on Windows; upgrade when the Windows check runs
fn run_claude(args: &[&str]) -> Result<()> {
    let out = Command::new("claude")
        .args(args)
        .stdin(Stdio::null())
        .output()
        .context("couldn't run `claude`; install Claude Code first")?;
    if !out.status.success() {
        bail!(
            "`claude {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// True when a file named `name` (plus `.exe` on Windows) sits in a PATH folder
pub fn on_path(name: &str, path: Option<&OsStr>) -> bool {
    let file = format!("{name}{}", env::consts::EXE_SUFFIX);
    path.is_some_and(|p| env::split_paths(p).any(|dir| dir.join(&file).is_file()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn on_path_finds_files_only() {
        let has = tempfile::tempdir().unwrap();
        let empty = tempfile::tempdir().unwrap();
        let exe = format!("brain{}", env::consts::EXE_SUFFIX);
        fs::write(has.path().join(&exe), "").unwrap();
        fs::create_dir(empty.path().join(&exe)).unwrap();

        let both = env::join_paths([empty.path(), has.path()]).unwrap();
        let only_empty = env::join_paths([empty.path()]).unwrap();

        assert!(on_path("brain", Some(&both)));
        assert!(
            !on_path("brain", Some(&only_empty)),
            "a folder named brain is not the program"
        );
        assert!(!on_path("brain", None));
    }
}
