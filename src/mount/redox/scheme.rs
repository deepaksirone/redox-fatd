use std::cell::RefCell;
use std::collections::BTreeMap;
use std::result::Result as StdResult;
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Read, Write, Seek};

use syscall::data::{Stat, StatVfs, TimeSpec};
use syscall::error::{Error, Result, EACCES, EEXIST, EISDIR, ENOTDIR, ENOTEMPTY, EPERM, ENOENT, EBADF, ELOOP, EINVAL};
use syscall::flag::{O_APPEND, O_CREAT, O_DIRECTORY, O_STAT, O_EXCL, O_TRUNC, O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, MODE_PERM, O_SYMLINK, O_NOFOLLOW};
use syscall::scheme::Scheme;

use SECTORSIZE;
//use fatfs::FileSystem;
use fat::FatFileSystem;

use super::resource::{Resource, DirResource, FileResource};
use super::spin::Mutex;

pub struct FileScheme<D: Read + Write + Seek> {
    name: String,
    fs: RefCell<FatFileSystem<D>>,
    next_id: AtomicUsize,
    files: Mutex<BTreeMap<usize, Box<Resource<D>>>>,
    // fmaps: Mutex<Fmaps> TODO
}

impl<D: Read + Write + Seek> FileScheme<D> {
    pub fn new(name: String, fs: FatFileSystem<D>) -> FileScheme<D> {
        FileScheme {
            name: name,
            fs: RefCell::new(fs),
            next_id: AtomicUsize::new(1),
            files: Mutex::new(BTreeMap::new()),
            // fmaps: Mutex::new(Fmaps::default()) TODO
        }
    }
}


fn example<D: Read + Write + Seek+ 'static>(fs_scheme: &'static mut FileScheme<D>, fs: FatFileSystem<D>) {
    //let mut fs_scheme = FileScheme::new(String::from("some"), fs);
    let bor_fs = fs_scheme.fs.borrow_mut();
    let mut root = bor_fs.root_dir().clone();
    let b = Box::new(DirResource::new(String::from("hehe"), 0, None, 0, root.clone()));
    fs_scheme.files.lock().insert(0, b);
}
