use crate::{arguments::Arguments, command, provenance};
use std::{
    path::Path,
    process::Command,
};

const MAX_RETRY: usize = 2;

pub(crate) struct Orchestrator;

impl Orchestrator {
    pub(crate) fn run(arguments: &Arguments) -> Result<(), String> {
        Self::verify_source(arguments)?;

        for retry in 0..=MAX_RETRY {
            Self::prepare_destination(&arguments.destination_directory)?;

            Self::generate(
                &arguments.source_directory,
                &arguments.destination_directory,
            )?;

            provenance::write(
                &arguments.destination_directory.join("provenance.ron"),
                &arguments.source_repository,
                &arguments.source_revision,
                &arguments.generator_version,
                &arguments.orchestrator_revision,
            )?;

            Self::stage(&arguments.destination_directory)?;

            if !Self::has_changes(&arguments.destination_directory)? {
                println!("generated data is already current");
                return Ok(());
            }

            Self::commit(
                &arguments.destination_directory,
                &arguments.source_revision,
            )?;

            if Self::push(&arguments.destination_directory)? {
                return Ok(());
            }

            if retry == MAX_RETRY {
                return Err(format!(
                    "failed to publish generated data after {} retries",
                    MAX_RETRY
                ));
            }

            eprintln!(
                "BIN main changed during publication; retrying from current origin/main ({}/{})",
                retry + 1,
                MAX_RETRY
            );
        }

        Err("unreachable publication state".to_owned())
    }

    fn verify_source(arguments: &Arguments) -> Result<(), String> {
        let revision = Self::git_output(
            &arguments.source_directory,
            &["rev-parse", "HEAD"],
        )?;

        if revision != arguments.source_revision {
            return Err(format!(
                "source revision mismatch: expected {}, got {}",
                arguments.source_revision,
                revision
            ));
        }

        Ok(())
    }

    fn prepare_destination(directory: &Path) -> Result<(), String> {
        Self::git_run(directory, &["fetch", "origin", "main"])?;
        Self::git_run(directory, &["reset", "--hard", "origin/main"])?;
        Self::git_run(directory, &["clean", "-fd"])?;
        Ok(())
    }

    fn generate(
        source_directory: &Path,
        destination_directory: &Path,
    ) -> Result<(), String> {
        command::run(
            Command::new("standardizer-generate-data")
                .arg("generate")
                .arg(source_directory)
                .arg(destination_directory),
        )
    }

    fn stage(directory: &Path) -> Result<(), String> {
        Self::git_run(directory, &["add", "-A"])
    }

    fn has_changes(directory: &Path) -> Result<bool, String> {
        Ok(!Self::git_output(
            directory,
            &["status", "--porcelain"],
        )?
        .is_empty())
    }

    fn commit(
        directory: &Path,
        source_revision: &str,
    ) -> Result<(), String> {
        let short_revision = source_revision
            .get(..12)
            .unwrap_or(source_revision);

        let message = format!(
            "chore: publish generated data from {short_revision}"
        );

        Self::git_run(
            directory,
            &["commit", "-m", &message],
        )
    }

    fn push(directory: &Path) -> Result<bool, String> {
        let mut command = Command::new("git");
        command
            .arg("-C")
            .arg(directory)
            .args(["push", "origin", "HEAD:main"]);

        command::succeeds(&mut command)
    }

    fn git_run(
        directory: &Path,
        arguments: &[&str],
    ) -> Result<(), String> {
        let mut command = Command::new("git");
        command
            .arg("-C")
            .arg(directory)
            .args(arguments);

        command::run(&mut command)
    }

    fn git_output(
        directory: &Path,
        arguments: &[&str],
    ) -> Result<String, String> {
        let mut command = Command::new("git");
        command
            .arg("-C")
            .arg(directory)
            .args(arguments);

        command::output(&mut command)
    }
}
