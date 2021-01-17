use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::SeekFrom;
pub struct SSTable {
    pub file: File,
    pub file_index: u8,
}

impl SSTable {
    fn read(offset: usize, size: usize) -> Result<Vec<u8>, Error> {
        let buf = Vec::with_capacity(size);

        Ok(buf)
    }

    fn scan(offset: usize, limit: usize) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let mut kvs = Vec::with_capacity(limit);
        Ok(kvs)
    }

    fn delete(&mut self, offset: usize) -> Result<(), Error> {
        self.file.seek(SeekFrom::Start(offset as u64))?;
        self.file.write(&[1]);
        Ok(())
    }
}
