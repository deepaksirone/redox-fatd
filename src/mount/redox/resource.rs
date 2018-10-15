//use std::cmp::{min, max};
//use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Read, Write, Seek};

use syscall::data::TimeSpec;
use syscall::error::{Result};
//use syscall::{Error, EBADF, EBUSY, EINVAL, EISDIR, EPERM}
//use syscall::flag::{O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, F_GETFL, F_SETFL, MODE_PERM};
//use syscall::{Stat, SEEK_SET, SEEK_CUR, SEEK_END};
use syscall::{Stat};

use partition::DiskPartition;

pub trait Resource<D: Read + Write + Seek> {
    fn block(&self) -> u64;
    fn dup(&self) -> Result<Box<Resource<D>>>;
    fn read(&mut self, buf: &mut [u8], fs: &mut DiskPartition) -> Result<usize>;
    fn write(&mut self, buf: &[u8], fs: &mut DiskPartition) -> Result<usize>;
    fn seek(&mut self, offset: usize, whence: usize, fs: &mut DiskPartition) -> Result<usize>;
    //TODO: Implement fmap
    //fn fmap(&mut self, offset: usize, size: usize, maps: &mut Fmaps, fs: &mut DiskPartition) -> Result<usize>;
    //fn funmap(&mut self, maps: &mut Fmaps, fs: &mut DiskPartition) -> Result<usize>;
    fn fchmod(&mut self, mode: u16, fs: &mut DiskPartition) -> Result<usize>;
    fn fchown(&mut self, uid: u32, gid: u32, fs: &mut DiskPartition) -> Result<usize>;
    fn fcntl(&mut self, cmd: usize, arg: usize) -> Result<usize>;
    fn path(&self, buf: &mut [u8]) -> Result<usize>;
    fn stat(&self, _stat: &mut Stat, fs: &mut DiskPartition) -> Result<usize>;
    //fn sync(&mut self, maps: &mut Fmaps, fs: &mut DiskPartition) -> Result<usize>;
    fn truncate(&mut self, len: usize, fs: &mut DiskPartition) -> Result<usize>;
    fn utimens(&mut self, times: &[TimeSpec], fs: &mut DiskPartition) -> Result<usize>;
}


