use std::fs;
use std::path::{Path, PathBuf};

// Bytes available on boot; Claude hooks cap out around 10,000
const BUDGET: usize = 9000;

///One section of the boot context, printed by brain boot <part>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Index,
    Profile,
    State,
}

impl Part {
    fn label(self) -> (usize, &'static str) {
        match self {
            Part::Index => (1, "index"),
            Part::Profile => (2, "profile"),
            Part::State => (3, "state"),
        }
    }
}

pub fn build_part(part: Part, vault: &Path) -> String {
    let agent = vault.join("Agent");
    let (n, name) = part.label();
    let mut out = format!("Engram boot context (part {n} of 4: {name})\n\n");

    match part {
        Part::Index => add_file(&mut out, "Vault Map", &agent.join("Vault Map.md")),
        Part::Profile => {
            add_file(&mut out, "Owner Profile", &agent.join("Owner Profile.md"));
            add_file(&mut out, "Vault Settings", &agent.join("Vault Settings.md"));
        }
        Part::State => add_file(
            &mut out,
            "Active Priorities",
            &agent.join("Active Priorities.md"),
        ),
    }

    out
}

/// If vault is missing, agent requests the path of the vault or where to create new vault
pub fn missing_vault(reason: &str) -> String {
    let (n, name) = Part::Index.label();

    format!(
        "Engram boot context (part {n} of 4: {name})\n\n{reason}. Ask the owner where their vault is, then run `brain init <PATH>`. \n"
    )
}

/// The Index block of the latest log before today, then today's log
pub fn build_today(vault: &Path, today: &str) -> String {
    let logs = log_files(&vault.join("Agent").join("Log"));
    let mut out = String::from("Engram boot context (part 4 of 4: today)\n\n");

    match logs.iter().rev().find(|(day, _)| day.as_str() < today) {
        Some((day, path)) => add_log(&mut out, &format!("Most recent log ({day})"), path),
        None => out.push_str("No earlier log.\n\n"),
    }
    match logs.iter().find(|(day, _)| day.as_str() == today) {
        Some((day, path)) => add_log(&mut out, &format!("Today so far ({day})"), path),
        None => out.push_str("No log yet today. This is the first session.\n\n"),
    }

    out
}

// Dated logs under Agent/Log/<month>/, oldest first; symlinks and other files skipped
fn log_files(log_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut logs = Vec::new();
    let Ok(months) = fs::read_dir(log_dir) else {
        return logs;
    };
    for month in months.flatten() {
        if !month.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let Ok(days) = fs::read_dir(month.path()) else {
            continue;
        };
        for day in days.flatten() {
            let path = day.path();
            if day.file_type().is_ok_and(|t| t.is_file())
                && path.extension().is_some_and(|ext| ext == "md")
                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                && is_log_name(stem)
            {
                logs.push((stem.to_string(), path))
            }
        }
    }
    logs.sort();
    logs
}

fn add_log(part: &mut String, title: &str, path: &Path) {
    let Ok(text) = fs::read_to_string(path) else {
        part.push_str(&format!("Couldn't read {title} at {}\n\n", path.display()));
        return;
    };
    match index_block(&text) {
        Some(block) => push_section(part, title, &block, path),
        None => part.push_str(&format!(
            "{title} has no ## Index block. Read {} directly.\n\n",
            path.display()
        )),
    }
}

fn is_log_name(stem: &str) -> bool {
    stem.len() == 10
        && stem.bytes().enumerate().all(|(i, b)| {
            if i == 4 || i == 7 {
                b == b'-'
            } else {
                b.is_ascii_digit()
            }
        })
}

fn index_block(text: &str) -> Option<String> {
    let mut lines = text.lines();
    lines.find(|line| line.trim_end() == "## Index")?;

    let block: Vec<&str> = lines.take_while(|line| !line.starts_with("## ")).collect();
    Some(block.join("\n").trim().to_string())
}
// Appends a titled section unless it would push the part over BUDGET
fn push_section(part: &mut String, title: &str, body: &str, path: &Path) {
    let section = format!("## {title}\n\n{body}\n\n");
    if part.len() + section.len() <= BUDGET {
        part.push_str(&section);
    } else {
        part.push_str(&format!(
            "OVER BUDGET: {title} ({} bytes) left out. Read {} directly.\n\n",
            section.len(),
            path.display()
        ));
    }
}

// Reads a vault file into the part; a failed read becomes a notice.
fn add_file(part: &mut String, title: &str, path: &Path) {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => {
            part.push_str(&format!("Couldn't read {title} at {}\n\n", path.display()));
            return;
        }
    };
    push_section(part, title, &text, path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_file_keeps_fits_flags_overflow_and_missing() {
        let dir = tempfile::tempdir().unwrap();
        let small = dir.path().join("small.md");
        let big = dir.path().join("big.md");

        fs::write(&small, "hello").unwrap();
        fs::write(&big, "x".repeat(BUDGET)).unwrap();

        let mut part = String::new();
        add_file(&mut part, "Small", &small);
        assert!(part.contains("## Small\n\nhello"));

        add_file(&mut part, "Big", &big);
        assert!(part.contains("OVER BUDGET: Big"));
        assert!(part.len() <= BUDGET);

        add_file(&mut part, "Gone", &dir.path().join("gone.md"));
        assert!(part.contains("Couldn't read Gone"));
    }

    #[test]
    fn profile_part_has_header_and_both_files() {
        let dir = tempfile::tempdir().unwrap();
        let agent = dir.path().join("Agent");

        fs::create_dir(&agent).unwrap();

        fs::write(agent.join("Owner Profile.md"), "owner").unwrap();
        fs::write(agent.join("Vault Settings.md"), "settings").unwrap();

        let out = build_part(Part::Profile, dir.path());
        assert!(out.starts_with("Engram boot context (part 2 of 4: profile)"));
        assert!(out.contains("## Owner Profile\n\nowner"));
        assert!(out.contains("## Vault Settings\n\nsettings"));
    }

    #[test]
    fn missing_file_reports_instead_of_panicking() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("Agent")).unwrap();

        let out = build_part(Part::State, dir.path());
        assert!(out.starts_with("Engram boot context (part 3 of 4: state)"));
        assert!(out.contains("Couldn't read Active Priorities"));
    }

    #[test]
    fn missing_vault_keeps_the_header() {
        let out = missing_vault("No vault set up yet");
        assert!(out.starts_with("Engram boot context (part 1 of 4: index)"));
        assert!(out.contains("No vault set up yet. Ask the owner"));
    }

    #[test]
    fn log_names_must_be_dates() {
        assert!(is_log_name("2026-11-22"));
        for bad in ["Log", "2026-1-22", "2026_11_22", "2026-11-22 copy"] {
            assert!(!is_log_name(bad), "{bad} should not count as a log")
        }
    }

    #[test]
    fn index_block_stops_at_next_heading() {
        let log = "# Sunday\n\n## Index\n\n- **A**: one\n- **B**: two\n\n## Session 1\nbody";
        let want = Some("- **A**: one\n- **B**: two".to_string());

        assert_eq!(index_block(log), want);
        assert_eq!(index_block(&log.replace('\n', "\r\n")), want);
        assert_eq!(index_block("# Sunday\n\n## Session 1\nbody"), None);
    }

    #[test]
    fn today_part_shows_last_log_and_today() {
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("Agent").join("Log");
        fs::create_dir_all(log.join("2026-10")).unwrap();
        fs::create_dir_all(log.join("2026-11")).unwrap();
        let entry = |text: &str| format!("# Day\n\n## Index\n\n- {text}\n\n## Session 1\nbody");
        fs::write(log.join("2026-10").join("2026-10-31.md"), entry("october")).unwrap();
        fs::write(log.join("2026-11").join("2026-11-20.md"), entry("older")).unwrap();
        fs::write(
            log.join("2026-11").join("2026-11-21.md"),
            entry("yesterday"),
        )
        .unwrap();
        fs::write(log.join("2026-11").join("2026-11-22.md"), entry("today")).unwrap();
        fs::write(log.join("2026-11").join("notes.md"), entry("junk")).unwrap();
        fs::write(log.join("Log.md"), "not a log").unwrap();

        let out = build_today(dir.path(), "2026-11-22");
        assert!(out.starts_with("Engram boot context (part 4 of 4: today)"));
        assert!(out.contains("## Most recent log (2026-11-21)\n\n- yesterday"));
        assert!(out.contains("## Today so far (2026-11-22)\n\n- today"));
        for skipped in ["older", "october", "junk", "Session 1"] {
            assert!(
                !out.contains(skipped),
                "{skipped} should not be in the part"
            );
        }

        // First session of a new month: last month's log, nothing yet today
        let next = build_today(dir.path(), "2026-12-01");
        assert!(next.contains("## Most recent log (2026-11-22)"));
        assert!(next.contains("No log yet today."));
    }

    #[test]
    fn today_part_without_logs_says_so() {
        let dir = tempfile::tempdir().unwrap();
        let out = build_today(dir.path(), "2026-11-22");
        assert!(out.contains("No earlier log."));
        assert!(out.contains("No log yet today."));
    }
}
