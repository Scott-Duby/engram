use std::fs;
use std::path::Path;
use std::process::{Command, Output};

/// Runs the real `brain boot <part>` with HOME and ENGRAM_VAULT pinned, so the owner's own config is never read.
fn boot(part: &str, home: &Path, vault: Option<&Path>) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_brain"));
    cmd.args(["boot", part]).env("HOME", home);
    match vault {
        Some(v) => cmd.env("ENGRAM_VAULT", v),
        None => cmd.env_remove("ENGRAM_VAULT"),
    };
    cmd.output().unwrap()
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn index_prints_the_vault_map() {
    let home = tempfile::tempdir().unwrap();
    let vault = tempfile::tempdir().unwrap();
    fs::create_dir(vault.path().join("Agent")).unwrap();
    fs::write(vault.path().join("Agent/Vault Map.md"), "map").unwrap();

    let out = boot("index", home.path(), Some(vault.path()));
    assert!(out.status.success());
    assert!(stdout(&out).starts_with("Engram boot context (part 1 of 4: index)"));
    assert!(stdout(&out).contains("## Vault Map\n\nmap"));
}

#[test]
fn missing_vault_folder_is_reported_once_and_never_fails() {
    let home = tempfile::tempdir().unwrap();
    let gone = home.path().join("gone");

    let index = boot("index", home.path(), Some(&gone));
    assert!(index.status.success());
    assert!(stdout(&index).contains("The vault couldn't be loaded"));

    let profile = boot("profile", home.path(), Some(&gone));
    assert!(profile.status.success());
    assert!(stdout(&profile).is_empty());
}

#[test]
fn today_prints_todays_log() {
    let home = tempfile::tempdir().unwrap();
    let vault = tempfile::tempdir().unwrap();
    let today = jiff::ZonedDateTime::now().date().to_string();
    let month = vault.path().join("Agent/Log").join(&today[..7]);
    fs::create_dir_all(&month).unwrap();
    fs::write(month.join(format!("{today}.md")), "## Index\n\n- today\n").unwrap();

    let out = boot("today", home.path(), Some(vault.path()));
    assert!(out.status.success());
    assert!(stdout(&out).starts_with("Engram boot context (part 4 of 4: today)"));
    assert!(stdout(&out).contains(&format!("## Today so far ({today})\n\n- today")));
}

// shortcut: Unix only, since `dirs` ignores HOME on Windows and the test would read the real config; upgrade if config_file gains an override.
#[cfg(unix)]
#[test]
fn no_vault_set_up_asks_for_the_path() {
    let home = tempfile::tempdir().unwrap();

    let out = boot("index", home.path(), None);
    assert!(out.status.success());
    assert!(stdout(&out).contains("No vault set up yet. Ask the owner"));
}

#[test]
fn unknown_part_is_rejected_loudly() {
    let home = tempfile::tempdir().unwrap();

    let out = boot("nope", home.path(), None);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr)
            .contains("possible values: index, profile, state, today")
    );
}
