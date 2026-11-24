use ron::ser::PrettyConfig;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Serialize)]
struct Provenance<'a> {
    schema_version: u32,
    source_repository: &'a str,
    source_revision: &'a str,
    generator: &'static str,
    generator_version: &'a str,
    orchestrator_revision: &'a str,
}

pub(crate) fn write(
    path: &Path,
    source_repository: &str,
    source_revision: &str,
    generator_version: &str,
    orchestrator_revision: &str,
) -> Result<(), String> {
    let provenance = Provenance {
        schema_version: 1,
        source_repository,
        source_revision,
        generator: "standardizer-generate-data",
        generator_version,
        orchestrator_revision,
    };

    let content = ron::ser::to_string_pretty(
        &provenance,
        PrettyConfig::new(),
    )
    .map_err(|error| format!("failed to serialize provenance: {error}"))?;

    fs::write(path, format!("{content}\n"))
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}
