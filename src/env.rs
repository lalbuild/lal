use std::{path::Path, process::Command, vec::Vec};

use super::{CliError, Config, Environment, LalResult, Manifest, StickyOptions};

/// Pull the current environment from docker
pub fn update(component_dir: &Path, environment: &Environment, env: &str) -> LalResult<()> {
    info!("Updating {} container", env);

    match environment {
        Environment::Container(container) => {
            let args: Vec<String> = vec!["pull".into(), format!("{}", container)];
            trace!("Docker pull {}", container);
            let s = Command::new("docker")
                .args(&args)
                .current_dir(&component_dir)
                .status()?;
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
pub fn set(component_dir: &Path, opts_: &StickyOptions, cfg: &Config, mf: &Manifest, env: &str) -> LalResult<()> {
    if !cfg.environments.contains_key(env) && !mf.environments.contains_key(env) {
        return Err(CliError::MissingEnvironment(env.into()));
    }
    // mutate a temporary copy - lal binary is done after this function anyway
    let mut opts = opts_.clone();
    opts.env = Some(env.into());
    opts.write(&component_dir)?;
    Ok(())
}

/// Clears the local .lal/opts file
pub fn clear(component_dir: &Path) -> LalResult<()> {
    let _ = StickyOptions::delete_local(&component_dir);
    Ok(())
}
