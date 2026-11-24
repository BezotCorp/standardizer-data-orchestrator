mod arguments;
mod command;
mod orchestrator;
mod provenance;

use arguments::Arguments;
use orchestrator::Orchestrator;
use std::process::ExitCode;

fn main() -> ExitCode {
    match Arguments::parse().and_then(|arguments| Orchestrator::run(&arguments)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("standardizer-data-orchestrator: {error}");
            ExitCode::FAILURE
        }
    }
}
