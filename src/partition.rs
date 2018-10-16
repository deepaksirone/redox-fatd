// The glue that binds DiskPartition to fatfs
use std::io::{Read, Write, Seek, SeekFrom};
use std::cmp::{min, max};
use std::fs::File;
use std::path::PathBuf;
//use syscall::error::{Error, Result, EIO};
use std::io::{Result, Error, ErrorKind};
use mbr::partition::{Partition, read_partitions};
use SECTORSIZE;

macro_rules! try_disk {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            return Err(Error::new(ErrorKind::Other, err));
        }
    })
}

pub fn get_partitions(filepath: PathBuf, typ: u8) -> Result<Vec<Partition>> {
    match read_partitions(filepath) {
        Ok(vec) => Ok(vec.into_iter().filter(|part| part.p_type == typ).collect()),
        Err(e) => { 
            eprintln!("Error reading partitions: {}", e);
            Err(e)
        }
    }
}

#[derive(Debug)]
pub struct DiskPartition<D: Read + Write + Seek> {
    diskfile: D,
    partition: Partition,
    byte_offset: u64,
}

impl<D: Read + Write + Seek>  DiskPartition<D> {
    pub fn new(disk: D, part: Partition) -> Self {
        DiskPartition {
            diskfile: disk,
            partition: part,
            byte_offset: 0
        }
    }
    fn get_size(&self) -> u32 {
        self.partition.p_size * SECTORSIZE
    }
}

impl<D: Read + Write + Seek>  Read for DiskPartition<D> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        try_disk!(self.diskfile.seek(SeekFrom::Start((SECTORSIZE * self.partition.p_lba) as u64 + self.byte_offset)));
        let count = try_disk!(self.diskfile.read(buf));
        Ok(count)
    }

}

impl<D: Read + Write + Seek>  Write for DiskPartition<D> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        try_disk!(self.diskfile.seek(SeekFrom::Start((SECTORSIZE * self.partition.p_lba) as u64 + self.byte_offset)));
        let count = try_disk!(self.diskfile.write(buf));
        Ok(count)
    }

    fn flush(&mut self) -> Result<()> {
        self.diskfile.flush()
    }
}

impl<D: Read + Write + Seek>  Seek for DiskPartition<D> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        
        self.byte_offset = match pos {
            SeekFrom::Current(off)  => max(0, min(self.get_size() as i64, self.byte_offset as i64 + off)) as u64,
            SeekFrom::Start(off)    => max(0, min(self.get_size() as u64, off)) as u64,
            SeekFrom::End(off)      => max(0, min(self.get_size() as i64, self.get_size() as i64 + off)) as u64
        };

        Ok(self.byte_offset)
    }

}
