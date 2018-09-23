use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use syscall::error::{Error, Result, EIO};
use mbr::partition::{Partition, read_partitions, table_dump};
use SECTORSIZE;

macro_rules! try_disk {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Disk I/O Error: {}", err);
            return Err(Error::new(EIO));
        }
    })
}

pub fn get_partitions(filepath: PathBuf, typ: u8) -> Option<Vec<Partition>> {
    match get_partitions(filepath) {
        Ok(vec) => Some(vec.into_iter().filter(|part| part.p_type == typ).collect()),
        Err(e) => { 
            eprintln!("Error reading partitions: {}", err);
            None
        }
    }
}

struct Fat32Partition {
    diskfile: File,
    paritition: Partition,
    byte_offset: u64
};

impl Fat32Partition {
    fn new(disk: File, part: Partition) -> Self {
        Fat32Partition {
            diskfile: disk,
            partition: part,
            byte_offset: 0
        }
    }
}

impl Read for Fat32Partition {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        try_disk!(self.diskfile.seek(SeekFrom::Start(SECTORSIZE * self.paritition.p_lba + self.byte_offset)));
        let count = try_disk!(self.diskfile.read(buf));
        Ok(count)
    }

}

impl Write for Fat32Partition {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        try_disk!(self.diskfile.seek(SeekFrom::Start(SECTORSIZE * self.paritition.p_lba + self.byte_offset)));
        let count = try_disk!(self.diskfile.write(buf));
        Ok(count)
    }
}

impl Seek for Fat32Partition {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        
        let new_byte_offset = match pos {
            SeekFrom::Current(off) => ((byte_offset as i64) + off) as u64,
            SeekFrom::Start(off) => off,
            SeekFrom::End(off) => ((self.partition.p_size * SECTORSIZE) as i64 + off) as u64
        };
        self.byte_offset = new_byte_offset;

        Ok(new_byte_offset as usize)
    }
}
