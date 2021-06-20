pub use self::{
    docker::{docker_run, DockerRunFlags, ShellModes},
    native::native_run,
};
use std::sync::Mutex;

lazy_static! {
    static ref COMMAND_LOCK: Mutex<()> = Mutex::new(());
}

mod docker;
mod native;
