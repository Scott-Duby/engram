use std::fs;
use std::path::Path;

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

// If vault is missing, agent requests the path of the vault or where to create new vault
pub fn missing_vault(reason: &str) -> String {
    let (n, name) = Part::Index.label();

    format!(
        "Engram boot context (part {n} of 4: {name})\n\n{reason}. Ask the owner where their vault is, then run `brain init <PATH>`. \n"
    )
}

// Adds files to boot section unless files are > BUDGET
fn add_file(part: &mut String, title: &str, path: &Path) {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => {
            part.push_str(&format!("Couldn't read {title} at {}\n\n", path.display()));
            return;
        }
    };
    let section = format!("## {title}\n\n{text}\n\n");
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
}
