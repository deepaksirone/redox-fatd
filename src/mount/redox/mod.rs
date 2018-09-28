extern crate spin;

use syscall;
use std::io::{self};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;



pub fn mount<D: Read + Write + Seek, P: AsRef<Path>, F: FnMut()>(filesystem: D, mountpoint: &P, mut callback: F) -> io::Result<()> {
    let mountpoint = mountpoint.as_ref();
    let mut socket = File::create(":{}", mountpoint.display())?;

    callback();
    syscall::setrens(0, 0).expect("redox-fatd: failed to enter null namespace");

    //TODO Scheme stuff
}
