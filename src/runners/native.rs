use super::COMMAND_LOCK;
use crate::core::{CliError, LalResult};
use std::{path::Path, process::Command, vec::Vec};

/// Runs an arbitrary command natively, without containerization
pub fn native_run(mut command: Vec<String>, component_dir: &Path) -> LalResult<()> {
    let cmd = command.remove(0);
    let mut script_cmd = Command::new(cmd);

    // Take hold of the mutex before changing directory, and keep it until the
    // command has finished executing. This is probably only useful for tests
    // which are run in threads in the same process. Since a process can only
    // exist in a single directory, this represents a race condition.
    // unwrap() will poison the lock on panic, failing the script. This is the
    // desired behaviour in all cases, whether or not we are in tests.
    let _guard = COMMAND_LOCK.lock().unwrap();

    script_cmd.args(command).current_dir(&component_dir);
    let s = script_cmd.status()?;

    if !s.success() {
        return Err(CliError::SubprocessFailure(s.code().unwrap_or(1001)));
    }

    Ok(())
}
