use std::{env, path::PathBuf};

pub(crate) struct Arguments {
    pub(crate) source_directory: PathBuf,
    pub(crate) destination_directory: PathBuf,
    pub(crate) source_repository: String,
    pub(crate) source_revision: String,
    pub(crate) generator_version: String,
    pub(crate) orchestrator_revision: String,
}

impl Arguments {
    pub(crate) fn parse() -> Result<Self, String> {
        let mut arguments = env::args().skip(1);

        if arguments.next().as_deref() != Some("run") {
            return Err(Self::usage());
        }

        let mut source_directory = None;
        let mut destination_directory = None;
        let mut source_repository = None;
        let mut source_revision = None;
        let mut generator_version = None;
        let mut orchestrator_revision = None;

        while let Some(flag) = arguments.next() {
            let value = arguments
                .next()
                .ok_or_else(|| format!("missing value for {flag}"))?;

            match flag.as_str() {
                "--source-directory" => source_directory = Some(PathBuf::from(value)),
                "--destination-directory" => {
                    destination_directory = Some(PathBuf::from(value));
                }
                "--source-repository" => source_repository = Some(value),
                "--source-revision" => source_revision = Some(value),
                "--generator-version" => generator_version = Some(value),
                "--orchestrator-revision" => orchestrator_revision = Some(value),
                _ => return Err(format!("unknown argument: {flag}\n{}", Self::usage())),
            }
        }

        Ok(Self {
            source_directory: source_directory.ok_or("missing --source-directory")?,
            destination_directory: destination_directory
                .ok_or("missing --destination-directory")?,
            source_repository: source_repository.ok_or("missing --source-repository")?,
            source_revision: source_revision.ok_or("missing --source-revision")?,
            generator_version: generator_version.ok_or("missing --generator-version")?,
            orchestrator_revision: orchestrator_revision
                .ok_or("missing --orchestrator-revision")?,
        })
    }

    fn usage() -> String {
        [
            "usage:",
            "  standardizer-data-orchestrator run",
            "    --source-directory <path>",
            "    --destination-directory <path>",
            "    --source-repository <owner/repository>",
            "    --source-revision <sha>",
            "    --generator-version <version>",
            "    --orchestrator-revision <sha>",
        ]
        .join("\n")
    }
}
