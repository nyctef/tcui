use failure::{bail, Fallible};
use std::process::Command;
use std::str::from_utf8;

pub fn get_current_branch() -> Fallible<String> {
    let output = Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()?;
    if !output.status.success() {
        bail!(format!(
            "Failed to get current branch name (git exit code {})",
            output.status.code().unwrap_or(-1)
        ));
    }

    Ok(from_utf8(&output.stdout)?.into())
}
