extern crate spin;

use syscall;
use std::fs::File;
use std::io::{self, Read, Write, Seek};
use std::path::Path;

mod resource;
mod scheme;

pub fn mount<D: Read + Write + Seek, P: AsRef<Path>, F: FnMut()>(_filesystem: D, mountpoint: &P, mut callback: F) -> io::Result<()> {
    let mountpoint = mountpoint.as_ref();
    let mut _socket = File::create(format!(":{}", mountpoint.display()))?;

    callback();
    syscall::setrens(0, 0).expect("redox-fatd: failed to enter null namespace");
    Ok(())
    //TODO Scheme stuff
}
