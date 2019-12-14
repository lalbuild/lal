pub use self::{
    docker::{docker_run, DockerRunFlags, ShellModes},
    native::native_run,
};

mod docker;
mod native;
