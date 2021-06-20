use flate2::{write::GzEncoder, Compression};
use std::{fs::File, path::Path};
use tar::Builder;

use super::LalResult;

/// Helper for stash and build
pub fn tar(component_dir: &Path, tarball: &Path) -> LalResult<()> {
    info!("Taring OUTPUT");

    let tarball = File::create(tarball)?;
    let compressor = GzEncoder::new(tarball, Compression::default());
    let mut archive = Builder::new(compressor);

    // Don't dereference symlinks, archive them as-is.
    // Dereferencing symlinks makes the archive larger, as we will then store two copies  of the
    // same file. Additionally, if OUTPUT contains dangling symlinks, adding them to the archive
    // will fail with a NotFound error.
    archive.follow_symlinks(false);

    archive.append_dir_all(".", component_dir.join("OUTPUT"))?;

    Ok(())
}
