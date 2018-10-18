use std::cmp::{min, max};
//use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Read, Write, Seek};
use fatfs::{FileSystem, File};

use syscall::data::TimeSpec;
use syscall::error::{Result};
use syscall::{Error, EBADF, EBUSY, EINVAL, EISDIR, EPERM};
//use syscall::flag::{O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, F_GETFL, F_SETFL, MODE_PERM};
use syscall::{Stat, SEEK_SET, SEEK_CUR, SEEK_END};
//use syscall::{Stat};

use partition::DiskPartition;

pub trait Resource<D: Read + Write + Seek> {
    fn block(&self) -> u64;
    fn dup(&self) -> Result<Box<Resource<D>>>;
    fn read(&mut self, buf: &mut [u8], fs: &mut FileSystem<D>) -> Result<usize>;
    fn write(&mut self, buf: &[u8], fs: &mut FileSystem<D>) -> Result<usize>;
    fn seek(&mut self, offset: usize, whence: usize, fs: &mut FileSystem<D>) -> Result<usize>;
    //TODO: Implement fmap
    //fn fmap(&mut self, offset: usize, size: usize, maps: &mut Fmaps, fs: &mut DiskPartition) -> Result<usize>;
    //fn funmap(&mut self, maps: &mut Fmaps, fs: &mut DiskPartition) -> Result<usize>;
    fn fchmod(&mut self, mode: u16, fs: &mut FileSystem<D>) -> Result<usize>;
    fn fchown(&mut self, uid: u32, gid: u32, fs: &mut FileSystem<D>) -> Result<usize>;
    fn fcntl(&mut self, cmd: usize, arg: usize) -> Result<usize>;
    fn path(&self, buf: &mut [u8]) -> Result<usize>;
    fn stat(&self, _stat: &mut Stat, fs: &mut FileSystem<D>) -> Result<usize>;
    //fn sync(&mut self, maps: &mut Fmaps, fs: &mut DiskPartition) -> Result<usize>;
    fn truncate(&mut self, len: usize, fs: &mut FileSystem<D>) -> Result<usize>;
    fn utimens(&mut self, times: &[TimeSpec], fs: &mut FileSystem<D>) -> Result<usize>;
}

pub struct DirResource {
    path: String,
    block: u64,
    data: Option<Vec<u8>>,
    seek: usize,
    uid: u32,
}

impl DirResource {
    pub fn new(path: String, block: u64, data: Option<Vec<u8>>, uid: u32) -> DirResource {
        DirResource {
            path: path,
            block: block,
            data: data,
            seek: 0,
            uid: uid,
        }
    }
}

impl<D: Read + Write + Seek> Resource<D> for DirResource {
    fn block(&self) -> u64 {
        self.block
    }

    fn dup(&self) -> Result<Box<Resource<D>>> {
        Ok(Box::new(DirResource {
            path: self.path.clone(),
            block: self.block,
            data: self.data.clone(),
            seek: self.seek,
            uid: self.uid
        }))
    }

    fn read(&mut self, buf: &mut [u8], _fs: &mut FileSystem<D>) -> Result<usize> {
        let data = self.data.as_ref().ok_or(Error::new(EISDIR))?;
        let mut i = 0;
        while i < buf.len() && self.seek < data.len() {
            buf[i] = data[self.seek];
            i += 1;
            self.seek += 1;
        }
        Ok(i)
    }

    fn write(&mut self, _buf: &[u8], _fs: &mut FileSystem<D>) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    fn seek(&mut self, offset: usize, whence: usize, _fs: &mut FileSystem<D>) -> Result<usize> {
        let data = self.data.as_ref().ok_or(Error::new(EBADF))?;
        self.seek = match whence {
            SEEK_SET => max(0, min(data.len() as isize, offset as isize)) as usize,
            SEEK_CUR => max(0, min(data.len() as isize, self.seek as isize + offset as isize)) as usize,
            SEEK_END => max(0, min(data.len() as isize, data.len() as isize + offset as isize)) as usize,
            _ => return Err(Error::new(EINVAL))
        };

        Ok(self.seek)
    }

    fn fchmod(&mut self, mode: u16, fs: &mut FileSystem<D>) -> Result<usize> {
        Ok(0) // FAT32 does not support file permissions
    }

    fn fchown(&mut self, uid: u32, gid: u32, fs: &mut FileSystem<D>) -> Result<usize> {
        Ok(0) // FAT32 does not support file permissions
    }

    fn fcntl(&mut self, _cmd: usize, _arg: usize) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    fn path(&self, buf: &mut [u8]) -> Result<usize> {
        let path = self.path.as_bytes();

        let mut i = 0;
        while i < buf.len() && i < path.len() {
            buf[i] = path[i];
            i += 1;
        }

        Ok(i)
    }

    //TODO: Implement this using the FAT32 node metadata
    fn stat(&self, stat: &mut Stat, fs: &mut FileSystem<D>) -> Result<usize> {
       /* let node = fs.node(self.block)?;

        *stat = Stat {
            st_dev: 0, // TODO
            st_ino: node.0,
            st_mode: node.1.mode,
            st_nlink: 1,
            st_uid: node.1.uid,
            st_gid: node.1.gid,
            st_size: fs.node_len(self.block)?,
            st_mtime: node.1.mtime,
            st_mtime_nsec: node.1.mtime_nsec,
            st_ctime: node.1.ctime,
            st_ctime_nsec: node.1.ctime_nsec,
            ..Default::default()
        };
        */
        Ok(0)
    }

    fn truncate(&mut self, _len: usize, _fs: &mut FileSystem<D>) -> Result<usize> {
        Err(Error::new(EBADF))
    }

    fn utimens(&mut self, _times: &[TimeSpec], _fs: &mut FileSystem<D>) -> Result<usize> {
        Err(Error::new(EBADF))
    }
}

struct FileResource<'a, T: Read + Write + Seek + 'a> {
    file: File<'a, T>
}
