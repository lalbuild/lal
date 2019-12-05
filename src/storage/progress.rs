use indicatif::{ProgressBar, ProgressStyle};
use std::{
    io,
    io::{Read, Seek, SeekFrom},
};

/// Wrapper around a `Read` that reports the progress made.
///
/// Used to monitor slow IO readers
/// Unfortunately cannot use this with http client yet as it does not implement seek
pub struct ProgressReader<R: Read + Seek> {
    rdr: R,
    pb: ProgressBar,
}

impl<R: Read + Seek> ProgressReader<R> {
    pub fn new(mut rdr: R) -> io::Result<ProgressReader<R>> {
        let len = rdr.seek(SeekFrom::End(0))?;
        rdr.seek(SeekFrom::Start(0))?;
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::default_bar().template("{bar:40.green/black} {bytes}/{total_bytes} ({eta})"),
        );
        Ok(ProgressReader { rdr, pb })
    }
}

impl<R: Read + Seek> Read for ProgressReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let rv = self.rdr.read(buf)?;
        self.pb.inc(rv as u64);
        Ok(rv)
    }
}
