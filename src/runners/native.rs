use std::{path::Path, process::Command, vec::Vec};

use core::{CliError, LalResult};

/// Runs an arbitrary command natively, without containerization
pub fn native_run(mut command: Vec<String>, component_dir: &Path) -> LalResult<()> {
    let cmd = command.remove(0);
    let mut script_cmd = Command::new(cmd);
    script_cmd.args(command).current_dir(&component_dir);
    let s = script_cmd.status()?;

    if !s.success() {
        return Err(CliError::SubprocessFailure(s.code().unwrap_or(1001)));
    }

    Ok(())
}
