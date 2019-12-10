use std::{process::Command, vec::Vec};

use super::{CliError, Config, Environment, LalResult, StickyOptions};

/// Pull the current environment from docker
pub fn update(environment: &Environment, env: &str) -> LalResult<()> {
    info!("Updating {} container", env);
    let args: Vec<String> = vec!["pull".into(), format!("{}", environment)];

    match environment {
        Environment::Container(container) => {
            trace!("Docker pull {}", container);
            let s = Command::new("docker").args(&args).status()?;
            trace!("Exited docker");
            if !s.success() {
                return Err(CliError::SubprocessFailure(s.code().unwrap_or(1001)));
            }
        }
        Environment::None => {}
    }
    Ok(())
}

/// Creates and sets the environment in the local .lal/opts file
pub fn set(opts_: &StickyOptions, cfg: &Config, env: &str) -> LalResult<()> {
    if !cfg.environments.contains_key(env) {
        return Err(CliError::MissingEnvironment(env.into()));
    }
    // mutate a temporary copy - lal binary is done after this function anyway
    let mut opts = opts_.clone();
    opts.env = Some(env.into());
    opts.write()?;
    Ok(())
}

/// Clears the local .lal/opts file
pub fn clear() -> LalResult<()> {
    let _ = StickyOptions::delete_local();
    Ok(())
}
