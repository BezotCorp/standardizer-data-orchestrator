use std::process::Command;

pub(crate) fn run(command: &mut Command) -> Result<(), String> {
    let description = format!("{command:?}");
    let status = command
        .status()
        .map_err(|error| format!("failed to execute {description}: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{description} exited with {status}"))
    }
}

pub(crate) fn succeeds(command: &mut Command) -> Result<bool, String> {
    command
        .status()
        .map(|status| status.success())
        .map_err(|error| format!("failed to execute {command:?}: {error}"))
}

pub(crate) fn output(command: &mut Command) -> Result<String, String> {
    let description = format!("{command:?}");
    let output = command
        .output()
        .map_err(|error| format!("failed to execute {description}: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "{description} exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|error| format!("{description} returned invalid UTF-8: {error}"))
}
