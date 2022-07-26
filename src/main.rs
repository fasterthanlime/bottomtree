use std::process::Command;

use tracing::error;
use tracing_subscriber::EnvFilter;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("info".parse().unwrap())
                .from_env_lossy(),
        )
        .init();

    let commits = std::fs::read("/tmp/commits.txt")?;
    let commits = String::from_utf8(commits)?;

    for commit in commits.lines().flat_map(|l| l.split(' ')) {
        if commit.is_empty() {
            continue;
        }

        let output = Command::new("git")
            .arg("show")
            .arg("--no-patch")
            .arg("--format=\"%P\"")
            .arg(commit)
            .current_dir("/home/amos/bearcove/rust")
            .output()?;
        if !output.status.success() {
            error!(
                "Failed to get parents for commit {}: status {:?}",
                commit, output.status
            );
            continue;
        }
        let output = String::from_utf8_lossy(&output.stdout[..]);
        let num_parents = output.split(' ').count();
        if num_parents > 1 {
            // merge commit, skip
            continue;
        }
        println!("{}", commit);
    }

    Ok(())
}
